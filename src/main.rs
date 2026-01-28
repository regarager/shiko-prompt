use crate::prompt::{left, right};
use std::env::args;

mod config;
mod modules;
mod prompt;
mod utils;

fn main() {
    let mode = args().nth(1).unwrap_or(String::from("--left"));

    match mode.as_str() {
        "--left" => println!("{}", left()),
        "--right" => println!("{}", right()),
        m => println!("unknown option {m}"),
    }
}
