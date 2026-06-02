use crate::prompt::{left, right};
use std::env::args;

mod config;
mod config_loader;
mod modules;
mod prompt;
mod utils;

fn main() {
    let mode = args().nth(1).unwrap_or(String::from("--left"));

    match mode.as_str() {
        "left" => println!("{}", left()),
        "right" => println!("{}", right()),
        "init" => {
            println!(
                r#"
autoload -Uz vcs_info
autoload -Uz add-zsh-hook

build_prompt() {{
  PROMPT=$(shiko left)
  RPROMPT=$(shiko right)
}}

setopt prompt_subst

add-zsh-hook precmd build_prompt
add-zsh-hook chpwd build_prompt

build_prompt
        "#            )
        }
        m => println!("unknown option {m}"),
    }
}
