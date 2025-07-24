use std::env::args;

use crate::prompt::{left, right};

mod config;
mod cwd;
mod prompt;
mod util;
mod vcs;
mod venv;

fn main() {
    let mode = args().nth(1).unwrap_or(String::from("--left"));

    match mode.as_str() {
        "--left" => println!("{}", left()),
        "--right" => println!("{}", right()),
        _ => println!("error: unknown mode {}", mode),
    }
}
