//! Wing data structures
// std
use std::fs;
use std::path::Path;

// external
use handlebars::*; // glob import for now
use serde::{Deserialize, Serialize};
use serde_json::from_str;

/// Represents a Wing configuration file
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct WingConfig {
    pub rss: bool,
    pub site_map: bool,
    pub link_type: String,
    pub optimisation_level: String,
    pub templates: String,
    pub content: String,
}

impl Default for WingConfig {
    fn default() -> Self {
        WingConfig {
            rss: false,
            site_map: false,
            link_type: String::from("relative"),
            optimisation_level: String::from("low"),
            templates: String::from("templates/"),
            content: String::from("/content"),
        }
    }
}

impl WingConfig {
    /// Generates a new WingConfig, using the wing.json configuration file.  This is a temporary solution.
    pub fn new() -> Result<WingConfig, std::io::Error> {
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

pub struct BuildAssets {
    content: String,
    content_files: Vec<String>,
}

#[derive(Serialize)]
pub struct WingTemplateData {
    pub content: String,
}

pub struct WingTemplate {
    pub content: String,
    pub content_path: String,

    pub template_raw: String,

    pub completed: String,
    pub completed_file: String,
}

impl WingTemplate {
    pub fn new(
        template: &Path,
        content: &Path,
        config: &WingConfig,
    ) -> Result<WingTemplate, std::io::Error> {
        let content_data = fs::read_to_string(content.as_os_str())
            .expect(&format!("Failed to read content in file {:?}.", template));

        let completed_file_location = format!("site/{}/index.html", content.to_str().unwrap());

        let mut hb = Handlebars::new();

        let template_raw = fs::read_to_string(template)?;
        let template_name = template
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap();

        let template = hb.register_template_file(template_name, template);

        let completed = match hb.render(
            template_name,
            &WingTemplateData {
                content: content_data.to_owned(),
            },
        ) {
            Ok(s) => s,
            Err(e) => {
                println!("Failed to render template: {}", e);
                std::process::exit(1)
            }
        };

        Ok(WingTemplate {
            content_path: content.to_str().unwrap().to_string(),
            content: content_data,

            template_raw: template_raw,

            completed: completed,
            completed_file: completed_file_location,
        })
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
