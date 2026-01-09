use dirs::home_dir;

use crate::{
    config::CONFIG,
    util::{RESET, bold, darken, fg},
};
use std::env;

pub fn darken_prefix(prefix: &str) -> String {
    let color = darken(CONFIG.color1, CONFIG.cwd_darken_factor);

    format!("{}{}{}", fg(&color), prefix, RESET)
}

pub fn highlight_last(cwd: &str) -> String {
    let index = cwd.rfind("/").unwrap_or_default();

    let prefix = &cwd[0..index];

    if CONFIG.cwd_darken {
        format!(
            "{}{}{}",
            darken_prefix(prefix),
            fg(CONFIG.color1),
            bold(&cwd[index..])
        )
    } else {
        format!("{}{}", prefix, bold(&cwd[index..]))
    }
}

pub fn cwd_info() -> String {
    let cwd = match env::current_dir() {
        Ok(x) => x.to_str().unwrap().to_string(),
        Err(_) => String::from("(deleted)"),
    };

    let home = match home_dir() {
        Some(x) => x.to_str().unwrap().to_string(),
        None => return cwd,
    };

    if cwd.starts_with(&home) {
        format!("~{}", &cwd[home.len()..])
    } else {
        cwd
    }
}

pub fn section_cwd() -> String {
    let mut cwd = cwd_info();

    if CONFIG.cwd_highlight_last {
        cwd = highlight_last(&cwd);
    } else {
        cwd = bold(&cwd);
    }

    format!("{}{}", fg(CONFIG.color1), cwd)
}
