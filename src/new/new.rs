// std
use std::fs;
use std::path::Path;

pub fn generate_output_dir(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/site/", name)))
}

fn generate_content_dir(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/content/", name)))
}

fn generate_content_index(name: &str) -> Result<(), std::io::Error> {
    fs::write(Path::new(&format!("./{}/content/index.md", name)), "")
}

fn generate_template_dir(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/templates/", name)))
}

fn generate_template_index(name: &str) -> Result<(), std::io::Error> {
    fs::write(Path::new(&format!("./{}/templates/index.hbs", name)), "")
}

pub fn generate_new(name: &str) -> Result<(), std::io::Error> {
    fs::create_dir(Path::new(&format!("./{}/", name)))?;

    generate_output_dir(name)?;

    generate_content_dir(name)?;
    generate_content_index(name)?;

    generate_template_dir(name)?;
    generate_template_index(name)?;

    Ok(())
}
