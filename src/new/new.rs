// std
use std::fs;
use std::path::Path;

// external
use serde_json::to_string_pretty;

// local
use wsg::WingConfig;

static BASIC_TEMPLATE: &'static str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/static/new.html"));

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
        to_string_pretty(&WingConfig {
            ..Default::default()
        })?,
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
