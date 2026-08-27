use dirs::home_dir;
use std::env;

use crate::config::CONFIG;
use crate::utils::{bold, darken, fg};

pub fn get_cwd() -> String {
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
    let cwd = get_cwd();
    let index = cwd.rfind("/").unwrap_or_default();

    let dark = fg(&darken(&CONFIG.colors.cwd, CONFIG.cwd_darken));
    let prefix = format!("{}{}", dark, &cwd[..index]);

    let suffix = if CONFIG.cwd_bold_last {
        bold(&cwd[index..])
    } else {
        cwd[index..].to_string()
    };

    Some(format!("{}{}{}", prefix, fg(&CONFIG.colors.cwd), suffix))
}
