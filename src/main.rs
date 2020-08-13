// std
use std::fs;

// external
use clap::{App, Arg, SubCommand}; // local

// local
use wing_generate::WingConfig;

fn main() {
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
                .about("Builds your site")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS")),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("build") {
        println!("V: {:?}", v); // debug
        println!("Called build!");
    }
}
