// std
use std::path::Path;
use std::process::exit;

// crates
use hotwatch::{Event, Hotwatch};
use open::that;
use rocket::{
    config::{Config, Environment},
    http::ContentType,
    response::content::Content,
    Catcher, Request, Rocket, Route, *,
}; // todo: figure out where catch macros are and import them instead of using glob
use rocket_contrib::serve::StaticFiles;

// local
use wsg::{build, log};

static NOT_FOUND: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/static/not_found.html"
));

#[catch(404)]
fn not_found() -> Content<&'static str> {
    Content(ContentType::HTML, NOT_FOUND)
}

/// If `open` is set to true, the site will **not** be opened automatically.
pub fn init(open: bool, port: u16) {
    if Path::new("site/").is_dir() == false {
        log(
            &String::from("Failed to start watching as site directory doesn't exist."),
            "f",
        )
        .unwrap();
        exit(1);
    } else {
        build(None, None);

        let mut hw = Hotwatch::new().expect("Failed to initialise file watcher");
        hw.watch("./", |e: Event| {
            if let Event::Write(_path) = e {
                log(&String::from("to build site"), "starting").unwrap();
                build(None, None);
                log(&String::from("Rebuilt site!"), "s").unwrap();
            }
        })
        .expect("Failed to watch directory.");

        if open == false {
            that("http://localhost:8000").expect("Failed to open in browser.");
        }

        let rocket_config = Config::build(Environment::Production)
            .port(port)
            .finalize()
            .unwrap();

        rocket::custom(rocket_config)
            .mount("/static/", StaticFiles::from("static/").rank(-1))
            .mount("/", StaticFiles::from("site/"))
            .register(catchers![not_found])
            .launch();
    };
}
