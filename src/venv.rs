use std::env;

use crate::config::CONFIG;
use crate::util::fg;
use crate::icons;

fn get_venv() -> Option<String> {
    let venv = env::var("VIRTUAL_ENV_PROMPT").ok();
    let conda = env::var("CONDA_DEFAULT_ENV").ok();

    if venv.is_some() {
        venv
    } else if conda.is_some() {
        conda
    } else {
        None
    }
}

pub fn section_venv() -> String {
    match get_venv() {
        None => String::new(),
        Some(v) => format!("{}{} {v}", fg(&CONFIG.modules.venv), icons::ICON_VENV),
    }
}
