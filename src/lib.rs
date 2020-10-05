//! Wing core
// std
use std::fs;
use std::io::{stdout, BufWriter, Write};
use std::path::Path;

// external
use comrak::{markdown_to_html, ComrakOptions};
use crossterm::{
    execute,
    style::{style, Color, Print},
    terminal::SetTitle,
    Result,
};
use handlebars::*; // glob import for now
use serde::{Deserialize, Serialize};
use serde_json::from_str;

// local
mod utils;

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
}

impl Default for WingConfig {
    fn default() -> Self {
        WingConfig {
            rss: false,
            site_map: false,
            link_type: String::from("relative"),
            optimisation_level: String::from("none"),
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
        hb: &Handlebars,
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

        let mut options = ComrakOptions::default();
        options.render.unsafe_ = true;
        options.extension.strikethrough = true;
        options.extension.table = true;
        options.extension.tasklist = true;
        options.extension.superscript = true;
        options.extension.header_ids = Some("".into());
        options.extension.footnotes = true;
        options.extension.description_lists = true;

        let completed = match hb.render(
            "index",
            &WingTemplateData {
                content: markdown_to_html(&content_data, &options),
                items: index.clone(),
                current: content
                    .display()
                    .to_string()
                    .replacen("content\\", "", 1)
                    .replacen(".md", "", 1),
            },
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

/// Removes the previous site
pub fn delete_output_dir() -> std::io::Result<()> {
    if Path::new("./site/").is_dir() == true {
        fs::remove_dir_all(Path::new("./site/"))?;
    }

    Ok(())
}

pub fn log(message: &String, message_type: &str) -> Result<()> {
    match message_type {
        "f" => execute!(
            stdout(),
            SetTitle("Wing Error"),
            Print(style("ERROR      ").with(Color::Red)),
            Print(style(message)),
            Print("\n")
        ),
        "s" => execute!(
            stdout(),
            Print(style("SUCESS     ").with(Color::Green)),
            Print(style(message)),
            Print("\n")
        ),
        "i" => execute!(
            stdout(),
            SetTitle("Wing: Indexing"),
            Print(style("INDEXING   ").with(Color::Cyan)),
            Print(style(message)),
            Print("\n")
        ),
        "g" => execute!(
            stdout(),
            Print(style("GENERATING ").with(Color::Cyan)),
            Print(style(message)),
            Print("\n")
        ),
        "c" => execute!(
            stdout(),
            Print(style("COMPLETED  ").with(Color::Green)),
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
