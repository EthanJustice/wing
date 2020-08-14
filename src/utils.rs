// std
use std::fs;
use std::path::{Path, PathBuf};

pub fn generate_dir(p: &str) {
    if Path::new(p).is_dir() == false {
        fs::create_dir(Path::new(p)).expect(&format!("Failed to create directory: {}", p));
    }
}

pub fn get_working_directory() -> Result<PathBuf, ()> {
    Ok(std::env::current_dir().unwrap())
}