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
use utils::{generate_dir, get_working_directory};

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
    /// Generates a new WingConfig, using the wing.json configuration file.  This is a temporary solution.
    pub fn new() -> std::result::Result<WingConfig, std::io::Error> {
        let config_raw = fs::read_to_string(Path::new(&concat!(
            env!("CARGO_MANIFEST_DIR"),
            "\\wing.json"
        )));

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
}

/// Represents a template
pub struct WingTemplate {
    /// Raw MarkDown
    pub content: String,
    /// Path to raw MarkDown
    pub content_path: String,

    /// Raw template content
    pub template_raw: String,

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
    ) -> std::result::Result<WingTemplate, std::io::Error> {
        let content_file_path = format!(
            r"{}\content{}",
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

        let formatted_file = format!(r"site{}", content.display());

        let with_replaced_extension = formatted_file.replace(".md", ".html");
        let completed_file_location = Path::new(&with_replaced_extension);
        let content_new_path = content
            .to_str()
            .unwrap()
            .replace(r"\content\", r"\site\")
            .replace("index.md", "");
        generate_dir(&content_new_path);

        let mut hb = Handlebars::new();

        let template_path_complete = format!(
            "{}{}",
            get_working_directory().unwrap().display(),
            template.display()
        );
        let template_path_complete_as_path = Path::new(&template_path_complete);

        let template_raw = fs::read_to_string(template_path_complete_as_path)
            .expect(&format!("Failed to read {:?}", template));
        let template_name = template
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .expect("Failed to generate template.");

        hb.register_template_file(template_name, template_path_complete_as_path)
            .expect("Failed to register template.");

        let completed = match hb.render(
            template_name,
            &WingTemplateData {
                content: markdown_to_html(&content_data, &ComrakOptions::default()),
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
                content_path: content_new_path,
                content: content_data,

                template_raw: template_raw,

                completed: completed,
                completed_file: String::from(completed_file_location.to_str().unwrap()),
            }),

            Err(_) => {
                log(&format!("Failed to write template."), "f").unwrap();
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
            Print(style("ERROR: ").with(Color::Red)),
            Print(style(message))
        ),
        "s" => execute!(
            stdout(),
            Print(style("SUCESS: ").with(Color::Green)),
            Print(style(message))
        ),
        _ => Ok(()),
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
