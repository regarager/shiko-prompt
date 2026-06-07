use crate::prompt::{print_left, print_right};
use crate::zsh_init::zsh_init;
use std::env::args;

mod config;
mod config_loader;
mod modules;
mod prompt;
mod utils;
mod zsh_init;

fn main() {
    let mode = args().nth(1).unwrap_or(String::from("left"));

    match mode.as_str() {
        "left" => print_left(),
        "right" => print_right(),
        "init" => zsh_init(),
        m => println!("unknown option {m}"),
    }
}
