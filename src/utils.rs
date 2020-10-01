// std
use std::path::PathBuf;

pub fn get_working_directory() -> Result<PathBuf, ()> {
    Ok(std::env::current_dir().unwrap())
}
