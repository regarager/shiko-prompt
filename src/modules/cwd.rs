use dirs::home_dir;
use std::env;

use crate::config::CONFIG;
use crate::utils::text::{RESET, bold, darken, fg};

pub fn darken_prefix(cwd_prefix: &str) -> String {
    let config = &CONFIG.modules.directory;
    let color = darken(&config.color, CONFIG.cwd_darken_factor);

    format!("{}{}", fg(&color), cwd_prefix)
}

pub fn highlight_last(cwd: &str) -> String {
    let config = &CONFIG.modules.directory;
    let index = cwd.rfind("/").unwrap_or_default();

    let cwd_prefix = &cwd[0..index];

    if CONFIG.cwd_darken {
        format!(
            "{}{}{}",
            darken_prefix(cwd_prefix),
            fg(&config.color),
            bold(&cwd[index..])
        )
    } else {
        format!("{}{}", cwd_prefix, bold(&cwd[index..]))
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

pub fn section_cwd() -> Option<String> {
    // NOTE: in the future, add option to specify the color of the darkened part?
    let mut cwd = cwd_info();

    if CONFIG.cwd_highlight_last {
        cwd = highlight_last(&cwd);
    } else {
        cwd = bold(&cwd);
    }

    cwd.push_str(RESET);

    Some(cwd)
}
