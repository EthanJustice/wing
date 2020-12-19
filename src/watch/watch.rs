// std

// crates
use hotwatch::{Event, Hotwatch};
use rocket::{Catcher, Rocket, Route};
use rocket_contrib::serve::StaticFiles;

// local
use wing_generate::build;

pub fn init_watch() {
    let mut hw = Hotwatch::new().expect("Failed to initialise file watcher");
    hw.watch("content/", |e: Event| {
        if let Event::Write(_path) = e {
            build(None, None);
        }
    })
    .expect("Failed to watch directory.");
    rocket::ignite()
        .mount("/", StaticFiles::from("site/"))
        .launch();
}
