// std
use std::fs;
use std::path::Path;

// external
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Error};

#[derive(Deserialize, Serialize, Debug)]
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

        if let Ok(value) = config_raw {
            let config: WingConfig = from_str(value.as_str())?;

            Ok(config)
        } else if let Err(e) = config_raw {
            Err(e)
        } else {
            Ok(WingConfig {
                ..Default::default()
            })
        }
    }
}
