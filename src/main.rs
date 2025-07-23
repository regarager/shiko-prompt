use crate::{
    config::CONFIG,
    cwd::section_cwd,
    util::{RESET, fg},
    vcs::section_git,
};

mod config;
mod cwd;
mod util;
mod vcs;

fn main() {
    println!(
        "{}{}{}{}{} ",
        section_cwd(),
        section_git(),
        fg(CONFIG.color3),
        CONFIG.icon_arrow,
        RESET
    );
}
