use std::env;
use std::env::args;
use std::fs;
use std::path::PathBuf;

use crate::prompt::{left, right};

mod config;
mod cwd;
mod icons;
mod prompt;
mod util;
mod vcs;
mod venv;

fn debug() {
    let file = args().nth(2).unwrap();
    let config_json = PathBuf::from(file.clone())
        .canonicalize()
        .expect("file not found");
    let config_content = fs::read_to_string(&config_json).expect("failed to read config file");

    println!("{}", config_content);
}

fn main() {
    let mode = args().nth(1).unwrap_or(String::from("--left"));

    match mode.as_str() {
        "--left" => println!("{}", left()),
        "--right" => println!("{}", right()),
        _ => debug(),
    }
}
