use std::fs;
use std::io::prelude::*;
use std::path::Path;

use wsg::build;

// generated from https://jaspervdj.be/lorem-markdownum/
static FILE: &'static str = "# Me hodierna corpusque namque prodit

## Pudore origo Assaracus conveniant

Lorem markdownum plura comitique temeraria Thracis requievit lignum faciem
sagitta et [flammas](http://furta-harenosae.io/) multo vae studio, serpere;
reclusa. Nostri hoc cuncta tinguebat redit. Expalluit pecudes habet eram austri,
ego per, **vel**. Virumque erat per, fugante quid sinunt volucres flexisque
ictus erat hortanda nec delicuit sinu.

> Ante oleis Euros. Hos auget serpere. Ab tura modo velocibus vidit tollite
> tandemque de addiderat tamen, populosque dixit, inferius permisit Caeneus.
> Ille procul sine illi pingue infans, atque longa volui Alcmene laevum montis;
> cui.

## Nate paterna

Illius deducite, multifori summorum accessisse me natura manus numina; eripe
sinistra inmemor, **conata**. Functus et pater sanctius in fibula miseris,
paternum aequore potest et Apolline, Ascaniumque. Passa de spectas cum premunt e
silvis Achilleos **vagas** succinctis **nutrix nigrior iste** fuerat thalamos
spicula Cythereide certe, victus. Adgreditur quod Persephones fatus, numerare
convertit mea voces sed, **causa**? Cavis amo at protendens notam atria mansit;
tunc solum quoque quidem, pelagoque citra per.

## Coepit volutant Medusaei filis habebat vires

Nos fertur, sine herbae terrae matrem, **in** videres sororem cornibus,
capillis. Tuus videtur in nunc prohibent aetas et is necem dona est movit
amantem lacertis pedibus.

Illa non ostendit paravi reddere superfusis dedit inconsumpta tamen surrexere
concrescere riget dubiaque. *Ambit* qui dearum terraque timuere, quas quos in
saxa!
";

// modified from src/new/new.rs

static BASIC_TEMPLATE: &'static str = "
<!DOCTYPE html>
<html lang=\"en\">

    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>{{ current }}</title>
        <link rel=\"stylesheet\" type=\"text/css\" href=\"/static/index.css\" />
    </head>

    <body>
        {{ content }}
    </body>

</html>
";

/// Creates a new output directory, where the results of the build process will go
fn generate_output_dir(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/site/", name)))
}

/// Creates a new content directory, where the raw MarkDown to be converted will go
fn generate_content_dir(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/content/", name)))
}

/// Creates an initial index.md file for the content directory
fn generate_content_index(name: &str) -> Result<(), std::io::Error> {
    fs::write(Path::new(&format!("./{}/content/index.md", name)), "")
}

/// Creates a new template directory, where the templates will go
fn generate_template_dir(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/templates/", name)))
}

/// generates a default template from a string included at compile-time
fn generate_default_template(name: &str) -> Result<(), std::io::Error> {
    fs::write(
        Path::new(&format!("./{}/templates/index.html", name)),
        BASIC_TEMPLATE,
    )
}

/// Creates a new static content directory, where the static content (styling, scripts, etc.) will go
fn generate_static_dir(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/static/", name)))
}

/// Generates a CSS file in the static content directory
fn generate_static_css(name: &str) -> Result<(), std::io::Error> {
    fs::write(Path::new(&format!("./{}/static/index.css", name)), "")
}

/// generates a default configuration JSON file
fn generate_default_config(name: &str) -> Result<(), std::io::Error> {
    fs::write(
        Path::new(&format!("./{}/.wing", name)),
        "{
    \"rss\": false,
    \"siteMap\": false,
    \"linkType\": \"relative\",
    \"optimisationLevel\": \"none\",
    \"preScripts\": [],
    \"postScripts\": []
        }",
    )
}

/// Scaffolding that generates a new skeleton Wing site
pub fn generate_new(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/", name)))?;

    generate_output_dir(name)?;

    generate_content_dir(name)?;
    generate_content_index(name)?;

    generate_template_dir(name)?;
    generate_default_template(name)?;

    generate_static_dir(name)?;
    generate_static_css(name)?;

    generate_default_config(name)?;

    Ok(())
}

fn main() {
    let mut max = 10;

    match generate_new("benches") {
        Ok(()) => {
            std::env::set_current_dir(&"benches/").unwrap();
            for _results in 0..=4 {
                for i in 0..=max {
                    fs::write(format!("content/{}.md", i), FILE);
                }

                let build_timing = std::time::Instant::now();

                build(None, None);

                let mut results = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("../results.txt")
                    .unwrap();

                writeln!(
                    results,
                    "Finished ({}): {}ms ({}s)\n",
                    max,
                    build_timing.elapsed().as_millis()
                    build_timing.elapsed().as_secs()
                )
                .unwrap();

                fs::remove_dir_all("content/").unwrap();
                fs::create_dir("content/").unwrap();

                max = max * 10;
            }
        }
        Err(e) => panic!(e.to_string()),
    };
}
