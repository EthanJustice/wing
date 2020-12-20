//! Wing core
// std
use std::fs;
use std::io::{stdout, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::SystemTime;

// external
use chrono::prelude::{DateTime, Utc};
use crossterm::{
    execute,
    style::{style, Color, Print},
    terminal::SetTitle,
    Result,
};
use lazy_static::lazy_static;
use pulldown_cmark::{html, CowStr, Event, Options, Parser};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tera::{Context, Tera};
use walkdir::WalkDir;

// local

/// Represents a Wing configuration file
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct WingConfig {
    /// If `true`, generates an RSS feed
    pub rss: bool,
    /// If `true`, generate a .xml sitemap
    pub site_map: bool,
    /// Values: `absolute`, `relative`
    /// Determines the type of link to use in **all** files
    pub link_type: String,
    /// Values: `none`, `low`, `high`
    /// Determines the level of optimisation to run the new site through
    pub optimisation_level: String,
    /// Scripts to run before building. This can also run other build tools.
    pub pre_scripts: Vec<String>,
    /// Scripts to run after building. This can also run other build tools.
    pub post_scripts: Vec<String>,
}

impl Default for WingConfig {
    fn default() -> Self {
        WingConfig {
            rss: false,
            site_map: false,
            link_type: String::from("relative"),
            optimisation_level: String::from("none"),
            pre_scripts: vec![],
            post_scripts: vec![],
        }
    }
}

impl WingConfig {
    /// Generates a new WingConfig, using the .wing configuration file.  This is a temporary solution.
    pub fn new() -> std::result::Result<WingConfig, std::io::Error> {
        let config_raw = fs::read_to_string(Path::new(&".wing"));

        match config_raw {
            Ok(value) => {
                let config: WingConfig = from_str(value.as_str())?;

                Ok(config)
            }
            Err(e) => Err(e),
        }
    }
}

/// Custom templating data
#[derive(Serialize)]
pub struct WingTemplateData {
    /// Raw MarkDown
    pub content: String,
    /// List of item paths
    pub items: Vec<String>,
    /// current item
    pub current: String,
    /// frontmatter
    pub frontmatter: WingTemplateFrontmatter,
    /// Last time file was modified
    pub modified: String,
    /// Time file was created
    pub created: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WingTemplateFrontmatter {
    /// template to use
    pub template: String,
}

/// Represents a template
pub struct WingTemplate {
    /// Raw MarkDown
    pub content: String,
    /// Path to raw MarkDown
    pub content_path: String,
    /// Completed content (content + template)
    pub completed: String,
    /// Path to completed file
    pub completed_file: String,
}

impl WingTemplate {
    pub fn new(
        tera: &Tera,
        content: &Path,
        _config: &WingConfig,
        index: &Vec<String>,
    ) -> std::result::Result<(), std::io::Error> {
        let content_file_path = format!(
            r"{}\{}",
            std::env::current_dir()
                .expect("Cannot get current working directory.")
                .display(),
            content.display()
        );

        let content_file = Path::new(&content_file_path);

        let content_data = match fs::read_to_string(content_file) {
            Ok(s) => s,
            Err(e) => {
                log(
                    &format!(
                        "Failed to read content of {}: {}",
                        content_file.display(),
                        e
                    ),
                    "f",
                )
                .unwrap();
                std::process::exit(1)
            }
        };

        let with_replaced_extension = content.with_extension("html");

        let completed_file_location =
            Path::new("site\\").join(with_replaced_extension.strip_prefix("content/").expect(
                &format!(
                    "Failed to generate new content location.  Computed content location: {}",
                    with_replaced_extension.display()
                ),
            ));

        let parent = completed_file_location.parent().unwrap();
        if parent.is_dir() == false {
            fs::create_dir_all(parent)?;
        }

        let mut frontmatter = WingTemplateFrontmatter {
            template: String::new(),
        };

        let mut options = Options::empty();
        options.insert(Options::all());
        let parser = Parser::new_ext(content_data.as_str(), options).map(|event| {
            if let Event::Text(text) = event.clone() {
                if text.starts_with("template: ") && frontmatter.template.len() == 0 {
                    let raw_frontmatter: WingTemplateFrontmatter =
                        serde_yaml::from_str(text.to_string().as_str())
                            .expect("Couldn't read file frontmatter.");
                    frontmatter.template = raw_frontmatter.template;

                    Event::Html(CowStr::Borrowed(""))
                } else {
                    event
                }
            } else {
                event
            }
        });

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let mut created: DateTime<Utc> = DateTime::<Utc>::from(SystemTime::now());
        let mut modified: DateTime<Utc> = DateTime::<Utc>::from(SystemTime::now());
        if let Ok(meta) = fs::metadata(content) {
            created = DateTime::<Utc>::from(meta.created().unwrap_or(SystemTime::now()));
            modified = DateTime::<Utc>::from(meta.modified().unwrap_or(SystemTime::now()));
        }

        let ctx = &WingTemplateData {
            content: html_output,
            items: index.clone(),
            current: content
                .display()
                .to_string()
                .replacen("content\\", "", 1)
                .replacen(".md", "", 1),
            frontmatter: frontmatter.clone(),
            created: created.format("%Y-%m-%d %H:%M").to_string(),
            modified: modified.format("%Y-%m-%d %H:%M").to_string(),
        };

        let template = if frontmatter.template.len() == 0 {
            String::from("index")
        } else {
            frontmatter.template
        };

        let completed = match tera.render(
            format!("{}.html", template).as_str(),
            &Context::from_serialize(ctx).expect("Failed to write template"),
        ) {
            Ok(s) => s,
            Err(e) => {
                log(&format!("Failed to render template: {}", e), "f").unwrap();
                std::process::exit(1);
            }
        };

        let completed_bytes = completed.as_bytes();

        let file = fs::File::create(completed_file_location.to_owned())?;

        let mut stream = BufWriter::new(file);

        for i in 0..completed.len() {
            stream.write(&[completed_bytes[i]]).unwrap();
        }

        match stream.flush() {
            Ok(()) => Ok(()),

            Err(e) => {
                log(
                    &format!(
                        "Failed to write completed file in {}.  Error: {}",
                        completed_file_location.to_str().unwrap(),
                        e
                    ),
                    "f",
                )
                .unwrap();
                std::process::exit(1);
            }
        }
    }
}

// build a site
pub fn build(app: Option<&clap::ArgMatches>, total_timing: Option<std::time::Instant>) {
    if Path::new("./site/").is_dir() == true && app.is_some() == true {
        if app
            .unwrap()
            .subcommand_matches("build")
            .unwrap()
            .is_present("force")
            == false
        {
            log(
                &String::from("Existing site found, run with -f to force."),
                "f",
            )
            .unwrap();
            return;
        }
    }

    let wing_config = match WingConfig::new() {
        Ok(val) => val,
        Err(e) => {
            log(
                &format!("Using defaults for Wing config.  Error: {}", e),
                "f",
            )
            .unwrap();
            WingConfig {
                ..Default::default()
            }
        }
    };

    for script in wing_config.pre_scripts.iter() {
        let args: Vec<&str> = script.split("--").collect();
        match Command::new(script)
            .args(args)
            .stdout(Stdio::piped())
            .output()
        {
            Ok(_v) => continue,
            Err(e) => {
                log(
                    &format!("Pre-run script failed with error: \"{}\"", e.to_string()),
                    "f",
                )
                .unwrap();
                std::process::exit(1)
            }
        };
    }

    let mut previous_build_exists: bool = false;
    if Path::new("./site/").is_dir() == true && app.is_some() == true {
        if app
            .unwrap()
            .subcommand_matches("build")
            .unwrap()
            .is_present("force")
            == false
        {
            log(
                &String::from("Previous builds already exist, run with -f to overwrite."),
                "f",
            )
            .unwrap();
            std::process::exit(1);
        }

        previous_build_exists = true;
    }

    if Path::new("./site/").is_dir() == false {
        fs::create_dir(Path::new(&format!("./site/"))).unwrap();
    }

    lazy_static! {
        pub static ref TERA_TEMPLATES: Tera = {
            let mut tera = match Tera::new("templates/**/*") {
                Ok(t) => t,
                Err(error) => {
                    log(&format!("Failed to parse template(s): {}", error), "f").unwrap();
                    std::process::exit(1);
                }
            };
            tera.autoescape_on(vec![]);
            tera
        };
    };

    let mut file_index = Vec::new();
    for entry in WalkDir::new("content").min_depth(1) {
        let file = entry.expect("Failed to read file.");
        let path = file.path();
        if path.is_file() == true && path.extension().unwrap() == "md" {
            file_index.push(
                String::from(file.path().to_str().unwrap())
                    .replace("content\\", "")
                    .replace(".md", ""),
            );
        }
    }

    let index: std::cell::RefCell<Vec<_>> =
        std::cell::RefCell::new(WalkDir::new("content").min_depth(1).into_iter().collect());

    index.into_inner().into_par_iter().for_each(|entry| {
        let path = entry.unwrap().into_path();
        if path.is_file() == true && path.extension().unwrap() == "md" {
            match WingTemplate::new(&TERA_TEMPLATES, &path, &wing_config, &file_index) {
                Ok(_template) => {}
                Err(e) => {
                    log(&String::from(e.to_string()), "f").unwrap();
                }
            };
        }
    });

    if let Some(timing) = total_timing {
        log(
            &format!("completed building in {}ms", timing.elapsed().as_millis(),),
            "s",
        )
        .unwrap();
    }

    if previous_build_exists == true {
        for entry in WalkDir::new("site").min_depth(1) {
            let file = entry.expect("Failed to read file.");
            let path = file.path();
            if path.is_file() == true && path.extension().unwrap() == "html" {
                let path_normalised = String::from(file.path().to_str().unwrap())
                    .replace("site\\", "")
                    .replace(".html", "");
                if file_index.contains(&path_normalised) == false {
                    fs::remove_file(&path).unwrap();
                }
            }
        }
    }

    for script in wing_config.post_scripts.iter() {
        let args: Vec<&str> = script.split("--").collect();
        match Command::new(script)
            .args(args)
            .stdout(Stdio::piped())
            .output()
        {
            Ok(_v) => continue,
            Err(e) => {
                log(
                    &format!("Post-run script failed with error: \"{}\"", e.to_string()),
                    "f",
                )
                .unwrap();
                std::process::exit(1)
            }
        };
    }
}

pub fn log(message: &String, message_type: &str) -> Result<()> {
    match message_type {
        "f" => execute!(
            stdout(),
            SetTitle("Wing Error"),
            Print(style("Error      ").with(Color::Red)),
            Print(style(message)),
            Print("\n")
        ),
        "s" => execute!(
            stdout(),
            Print(style("Success    ").with(Color::Green)),
            Print(style(message)),
            Print("\n")
        ),
        "i" => execute!(
            stdout(),
            SetTitle("Indexing"),
            Print(style("Indexing   ").with(Color::Cyan)),
            Print(style(message)),
            Print("\n")
        ),
        "g" => execute!(
            stdout(),
            Print(style("Generating ").with(Color::Cyan)),
            Print(style(message)),
            Print("\n")
        ),
        "c" => execute!(
            stdout(),
            Print(style("Completed  ").with(Color::Green)),
            Print(style(message)),
            Print("\n")
        ),
        _ => execute!(stdout(), Print(style(message)), Print("\n")),
    }
}

#[cfg(test)]
mod tests {
    use super::WingConfig;
    #[test]
    pub fn test_config() {
        let from_fn = WingConfig::new();

        match from_fn {
            Ok(val) => {
                let defaults = WingConfig {
                    ..Default::default()
                };

                assert_eq!(defaults.rss, val.rss);
                assert_eq!(defaults.site_map, val.site_map);
                assert_eq!(defaults.link_type, val.link_type);
                assert_eq!(defaults.optimisation_level, val.optimisation_level);
                assert_eq!(defaults.templates, val.templates);
                assert_eq!(defaults.content, val.content);
            }
            Err(e) => {
                assert_eq!(true, false, "WingConfig::new() failed: {}", e);
            }
        }
    }
}
