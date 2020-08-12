// std
use std::fs;
use std::path::Path;

// external
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WingConfig {
    pub rss: bool,
    pub site_map: bool,
    pub link_type: String,
    pub optimisation_level: String,
}

impl Default for WingConfig {
    fn default() -> Self {
        WingConfig {
            rss: false,
            site_map: false,
            link_type: String::from("relative"),
            optimisation_level: String::from("low"),
        }
    }
}

impl WingConfig {
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
            }
            Err(e) => {
                assert_eq!(true, false, "WingConfig::new() failed: {}", e);
            }
        }
    }
}
