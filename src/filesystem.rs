#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{Read, Write};

// Path Type Enum
pub enum PathType {
    File,
    Directory,
    Invalid,
}

// Is something a file or directory?
pub fn path_type(path: &str) -> PathType {
    match path_exists(path) {
        true => {},
        false => return PathType::Invalid,
    };

    match Path::new(path).is_file() {
        true => return PathType::File,
        false => return PathType::Directory,
    };
}

// Check to see if a path exists.
pub fn path_exists(path: &str) -> bool {
    return Path::new(path).exists();
}

// Read a file.
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(file_contents);
}

// Write a file.
pub fn write_file(contents: &str, path: &str) -> Result<(), io::Error> {
    let mut file = match File::create(path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match file.write_all(contents.as_bytes()) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(());
}

// Return a list of items in a directory. (Returns full paths!)
pub fn list_directory(path: &str) -> Result<Vec<String>, io::Error> {
    let contents = match fs::read_dir(path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    let mut return_contents: Vec<String> = Vec::new();

    for i in contents {
        return_contents.push(i.unwrap().path().display().to_string());
    }

    return Ok(return_contents);
}

// Create a directory, and its parents.
pub fn create_directory(path: &str) -> Result<(), io::Error> {
    match fs::create_dir_all(path) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(());
}

pub fn remove_directory(path: &str) -> Result<(), io::Error> {
    match fs::remove_dir_all(path) {
        Ok(_o) => {},
        Err(e) => return Err(e),
    };

    return Ok(());
}
