// std
use std::path::Path;
use std::process::exit;

// crates
use hotwatch::{Event, Hotwatch};
use open::that;
use rocket::{Catcher, Rocket, Route};
use rocket_contrib::serve::StaticFiles;

// local
use wing_generate::{build, log};

/// If `open` is set to true, the site will **not** be opened automatically.
pub fn init_watch(open: bool) {
    if Path::new("site/").is_dir() == false {
        log(
            &String::from("Failed to start watching as site directory doesn't exist."),
            "f",
        )
        .unwrap();
        exit(1);
    } else {
        let mut hw = Hotwatch::new().expect("Failed to initialise file watcher");
        hw.watch("./", |e: Event| {
            if let Event::Write(_path) = e {
                build(None, None);
            }
        })
        .expect("Failed to watch directory.");

        if open == false {
            that("http://localhost:8000").expect("Failed to open in browser.");
        }

        rocket::ignite()
            .mount("/static/", StaticFiles::from("static/").rank(-1))
            .mount("/", StaticFiles::from("site/"))
            .launch();
        //    todo!("Add 404 template, maybe use Maud");
    };
}
