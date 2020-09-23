//! Wing is a static site generator which does everything in its power to be *very* fast.
// std
use std::fs;
use std::io::{stdout, Write};
use std::path::Path;
use std::time::Instant;

// external
use clap::{App, Arg, SubCommand};
use crossterm::{execute, terminal::SetTitle};
use walkdir::WalkDir;

// local
mod new;
use new::new::generate_new;

mod utils;

use wing_generate::{delete_output_dir, log, WingConfig, WingTemplate};

fn main() {
    let total_timing = Instant::now();
    delete_output_dir().expect("Failed to remove previous build artifacts."); // debug
    execute!(stdout(), SetTitle("Wing"));

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

    if let Some(_v) = app.subcommand_matches("build") {
        fs::create_dir(Path::new(&format!("./site/"))).unwrap();
        let index_timing = Instant::now();
        log(&String::from("content..."), "i").unwrap();
        let mut index = Vec::new();
        for entry in WalkDir::new("content").min_depth(1) {
            let file = entry.expect("Failed to read file.");
            let path = file.path();
            if path.is_file() == true && path.extension().unwrap() == "md" {
                index.push(
                    String::from(file.path().to_str().unwrap())
                        .replace("content\\", "")
                        .replace(".md", ""),
                );
            }
        }
        log(
            &format!(
                "content indexing in {}ms",
                index_timing.elapsed().as_millis()
            ),
            "c",
        )
        .unwrap();

        for entry in WalkDir::new("content").min_depth(1) {
            let entry_timing = Instant::now();
            let file = entry.expect("Failed to read file.");
            let path = file.path();
            if path.is_file() == true && path.extension().unwrap() == "md" {
                match WingTemplate::new(
                    Path::new(r"\templates\index.hbs"),
                    path,
                    &wing_config,
                    &index,
                ) {
                    Ok(_template) => {
                        log(
                            &format!(
                                "{} in {}ms",
                                path.to_str()
                                    .unwrap()
                                    .replacen("content\\", "", 1)
                                    .replacen(".md", "", 1),
                                entry_timing.elapsed().as_millis()
                            ),
                            "b",
                        )
                        .unwrap();
                    }
                    Err(e) => {
                        log(&String::from(e.to_string()), "f").unwrap();
                    }
                };
            }
        }
        log(
            &format!(
                "completed building in {}ms",
                total_timing.elapsed().as_millis()
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
