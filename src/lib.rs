//! Wing core
// std
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;

// external
use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions};
use crossterm::{
    execute,
    style::{style, Color, Print, StyledContent},
    terminal::SetTitle,
    Result,
};
use handlebars::*; // glob import for now
use serde::{Deserialize, Serialize};
use serde_json::from_str;

// local
mod utils;
use utils::get_working_directory;

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
        template: &Path,
        content: &Path,
        config: &WingConfig,
        index: &Vec<String>,
    ) -> std::result::Result<WingTemplate, std::io::Error> {
        let content_file_path = format!(
            r"{}\{}",
            get_working_directory()
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

        let mut hb = Handlebars::new();

        let template_path_complete = format!(
            "{}{}",
            get_working_directory().unwrap().display(),
            template.display()
        );

        let template_path_complete_as_path = Path::new(&template_path_complete);
        let template_name = template.file_stem().unwrap().to_str().unwrap();

        hb.register_template_file(template_name, template_path_complete_as_path)
            .expect("Failed to register template.");

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
            template_name,
            &WingTemplateData {
                content: markdown_to_html(&content_data, &options),
                items: index.clone(),
            },
        ) {
            Ok(s) => s,
            Err(e) => {
                log(&format!("Failed to render template: {}", e), "f").unwrap();
                std::process::exit(1);
            }
        };

        match fs::write(
            completed_file_location.to_owned(),
            completed.to_owned().as_bytes(),
        ) {
            Ok(()) => Ok(WingTemplate {
                content_path: String::from(parent.to_str().unwrap()),
                content: content_data,

                completed: completed,
                completed_file: String::from(completed_file_location.to_str().unwrap()),
            }),

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
        "b" => execute!(
            stdout(),
            Print(style("BUILT      ").with(Color::Yellow)),
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
