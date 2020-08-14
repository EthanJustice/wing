//! Wing is a static site generator which does everything in its power to be *very* fast.
// std
use std::fs;
use std::path::Path; // temp

// external
use clap::{App, Arg, SubCommand}; // local

// local
mod new;
use new::*;
use wing_generate::{delete_output_dir, generate_fs_structure, WingConfig, WingTemplate};

fn main() {
    delete_output_dir().expect("Failed to remove previous build artifacts."); // debug
    generate_fs_structure();

    let wing_config = match WingConfig::new() {
        Ok(val) => val,
        Err(e) => {
            println!("Using defaults for Wing config.  Error: {}", e);
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
                .author(env!("CARGO_PKG_AUTHORS")),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("build") {
        println!("Called build: {:?}", v); // debug
        let test = WingTemplate::new(
            // used for debugging for now
            &Path::new(r"\templates\index.hbs"),
            &Path::new(r"\index.md"),
            &wing_config,
        );
    } else if let Some(v) = app.subcommand_matches("new") {
        println!("Called new: {:?}", v);
    }
}
