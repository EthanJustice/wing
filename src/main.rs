//! Wing is a static site generator which does everything in its power to be *very* fast.
// std
use std::fs;
use std::path::Path; // temp

// external
use clap::{App, Arg, SubCommand};
use crossterm::style::{style, Color};
use walkdir::WalkDir;

// local
mod new;
use new::new::generate_new;

mod utils;
use utils::get_working_directory;

use wing_generate::{delete_output_dir, log, WingConfig, WingTemplate};

fn main() {
    delete_output_dir().expect("Failed to remove previous build artifacts."); // debug

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

    let app = App::new("wing")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds your site.")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS")),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new wing project.")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("name")
                        .allow_hyphen_values(true)
                        .default_value("site")
                        .help("The name of the site you want to create")
                        .number_of_values(1),
                ),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("build") {
        fs::create_dir(Path::new(&format!("./site/"))).unwrap();
        log(&String::from("Content..."), "i").unwrap();
        for entry in WalkDir::new("content").min_depth(1) {
            let file = entry.expect("Failed to read file.");
            let path = file.path();
            if path.is_file() == true && path.extension().unwrap() == "md" {
                WingTemplate::new(Path::new(r"\templates\index.hbs"), path, &wing_config);
            }
        }
    } else if let Some(v) = app.subcommand_matches("new") {
        log(&String::from("new project"), "g").unwrap();
        match generate_new(v.value_of("name").unwrap()) {
            Ok(()) => {
                log(
                    &format!("Created project {}!", v.value_of("name").unwrap()),
                    "s",
                )
                .unwrap();
            }
            Err(e) => {
                log(&format!("failed to create new project: {}", e), "f").unwrap();
                std::process::exit(1);
            }
        };
    }
}
