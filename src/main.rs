use crate::prompt::{left, right};
use std::env::args;

mod arrow;
mod config;
mod cwd;
mod icons;
mod prompt;
mod util;
mod vcs;
mod venv;

fn main() {
    let mode = args().nth(1).unwrap_or(String::from("--left"));

    match mode.as_str() {
        "--left" => println!("{}", left()),
        "--right" => println!("{}", right()),
        m => println!("unknown option {m}"),
    }
}
