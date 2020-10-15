//! Wing is a static site generator which does everything in its power to be *very* fast.
// std
use std::fs;
use std::path::Path;
use std::time::Instant;

// external
use clap::{App, Arg, SubCommand};
use lazy_static::lazy_static;
use rayon::prelude::*;
use tera::*; // glob import for now
use walkdir::WalkDir;

// local
mod new;
use new::new::generate_new;

mod utils;

use wing_generate::{delete_output_dir, log, WingConfig, WingTemplate};

fn main() {
    let total_timing = Instant::now();

    let app = App::new("wing")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds your site.")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .help("Deletes existing site, if any."),
                ),
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

    if let Some(_v) = app.subcommand_matches("build") {
        if Path::new("./site/").is_dir() == true
            && app.subcommand_matches("build").unwrap().is_present("force") == true
        {
            delete_output_dir().expect("Failed to remove previous build artifacts.");
        } else {
            log(
                &String::from("Existing site found, run with -f to force."),
                "f",
            )
            .unwrap();
            return;
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

        fs::create_dir(Path::new(&format!("./site/"))).unwrap();

        lazy_static! {
            pub static ref TERA_TEMPLATES: Tera = {
                let mut tera = match Tera::new("templates/**/*") {
                    Ok(t) => t,
                    Err(error) => {
                        log(&format!("Failed to parse template(s): {}", error), "f").unwrap();
                        std::process::exit(1);
                    }
                };
                tera.autoescape_on(vec!["html"]);
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

        log(
            &format!(
                "completed building in {}ms",
                total_timing.elapsed().as_millis(),
            ),
            "s",
        )
        .unwrap();
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
