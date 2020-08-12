// std
use std::fs;

// external
use clap::{App, Arg, SubCommand}; // local

// local
use wing_generate::WingConfig;

fn main() {
    let wing_config = match WingConfig::new() {
        Ok(val) => val,
        Err(e) => WingConfig {
            ..Default::default()
        },
    };

    let app = App::new("wing")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("generate")
                .about("Builds your wiki/site")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS")),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("generate") {
        println!("Called build!")
    }
}
