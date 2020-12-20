//! Wing is a static site generator which does everything in its power to be *very* fast.
// std
use std::fs;
use std::path::Path;
use std::time::Instant;

// external
use clap::{App, Arg, SubCommand};

// local
mod new;
use new::new::generate_new;

mod watch;
use watch::watch::init_watch;

use wing_generate::{build, log};

fn main() {
    let total_timing = Instant::now();

    let app = App::new("wing")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("watch")
                .about("Rebuild your site on file changes, and serve it.")
                .version(env!("CARGO_PKG_VERSION"))
                .author(env!("CARGO_PKG_AUTHORS"))
                .arg(
                    Arg::with_name("silent")
                        .long("silent")
                        .short("s")
                        .help("Prevents site being opened in the browser automatically."),
                )
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .short("p")
                        .help("The port to use.")
                        .required(false)
                        .takes_value(true),
                ),
        )
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
        build(Some(&app), Some(total_timing));
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
    } else if let Some(v) = app.subcommand_matches("watch") {
        let port: u16 = v.value_of("port").unwrap().parse().unwrap_or(8000);
        init_watch(v.is_present("silent"), port);
    }
}
