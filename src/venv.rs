use std::env;

use crate::config::CONFIG;
use crate::icons;
use crate::util::module_fmt;

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

pub fn section_venv() -> Option<String> {
    match get_venv() {
        Some(v) => Some(format!(
            "{}{} {v}",
            module_fmt(&CONFIG.modules.venv),
            icons::VENV
        )),
        None => None,
    }
}
