// std
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_working_directory() -> Result<PathBuf, ()> {
    Ok(std::env::current_dir().unwrap())
}
