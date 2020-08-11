// std
use std::fs;

// external
use clap::{App, Arg, SubCommand};

// local

fn main() {
    let app = App::new("wing")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds your wiki/site")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS")),
        )
        .get_matches();

    if let Some(v) = app.subcommand_matches("build") {
        println!("Called build!")
    }
}
