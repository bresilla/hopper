#![allow(dead_code)]

use std::io;
use std::process::Command;
use colored::Colorize;
use users::get_current_username;
use crate::convert::*;

#[derive(PartialEq)]
pub enum HistoryMode {
    Remove,
    Add,
}

pub struct History {
    pub mode: HistoryMode,
    pub line: String,
}

pub fn abort() { // Try not to use this function!
    std::process::exit(1);
}

pub fn run_command(command: &str) -> bool {
    return match Command::new("bash").args(["-c", command]).status() {
        Ok(o) => o,
        Err(_e) => return false,
    }.success();
}

pub fn cut(full: &str, fword: u32, dword: char) -> String {
    let vecced = str_to_string_vec(full, dword.to_string().as_str());

    let fspot = fword - 1;

    if vecced.len() < fword as usize {
        return String::from("");
    }

    return vecced[fspot as usize].to_string();
}

pub fn sed(full: &str, replace: &str, with: &str) -> String {
    let mut phrase_vec: Vec<String> = Vec::new();

    for i in full.split(replace) {
        phrase_vec.push(i.to_string());
    }

    let mut phrase = String::new();

    for i in 0..phrase_vec.len() {
        phrase.push_str(phrase_vec[i].as_str());

        if i < phrase_vec.len() - 1 {
            phrase.push_str(with);
        }
    }

    return phrase;
}

pub fn name_from_path(path: &str) -> String {
    let converted = str_to_string_vec(path, "/");

    return converted[converted.len() - 1].to_string();
}

pub fn custom_error(error: &str) -> io::Error {
    return io::Error::new(io::ErrorKind::Other, error);
}

pub fn is_root_user() -> bool {
    if username() == "root" {
        return true;
    } else {
        return false;
    }
}

pub fn username() -> String {
    let username: String = match get_current_username() {
        Some(uname) => uname.to_str().unwrap().to_string(),
        None => panic!("Unable to get username!"),
    };

    return username;
}

pub fn remove_string_vec_duplicates(dup_vec: &Vec<String>) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();

    for i in dup_vec.iter() {
        if new_vec.contains(i) == false {
            new_vec.push(i.to_string());
        }
    }

    return new_vec;
}

pub fn history(str_1: &str, str_2: &str) -> Vec<History> {
    let lines_1 = remove_string_vec_duplicates(&str_to_string_vec(str_1, "\n"));
    let lines_2 = remove_string_vec_duplicates(&str_to_string_vec(str_2, "\n"));

    let mut history_vec: Vec<History> = Vec::new();

    for i in lines_1.iter() {
        if i.trim() != "" {
            if lines_2.contains(i) == false {
                history_vec.push(History {
                    mode: HistoryMode::Remove,
                    line: i.to_string(),
                });
            }
        }
    }

    for i in lines_2.iter() {
        if i.trim() != "" {
            if lines_1.contains(i) == false {
                history_vec.push(History {
                    mode: HistoryMode::Add,
                    line: i.to_string(),
                });
            }
        }
    }

    return history_vec;
}

pub fn print_history(diff_vec: &Vec<History>) {
    for i in diff_vec.iter() {
        if i.mode == HistoryMode::Remove {
            println!("{}", format!("- {}", i.line).red().bold());
        }

        else if i.mode == HistoryMode::Add {
            println!("{}", format!("+ {}", i.line).green().bold());
        }

        else {
            println!("{}", format!("  {}", i.line));
        }
    }
}
