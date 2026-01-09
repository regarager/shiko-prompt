use crate::{
    config::CONFIG,
    cwd::section_cwd,
    icons,
    util::{RESET, fg},
    vcs::section_vcs_changes,
    vcs::section_vcs_branch,
    venv::section_venv,
};
use string_builder::Builder;

fn arrow() -> String {
    format!("{}{}{}", fg(CONFIG.color3), icons::ICON_ARROW, RESET)
}

pub fn left() -> String {
    let mut vec: Vec<String> = Vec::new();

    vec.push(section_cwd());
    vec.push(section_vcs_branch());
    vec.push(section_vcs_changes());
    if !CONFIG.venv_right_side {
        vec.push(section_venv());
    }
    vec.push(arrow());

    let mut builder = Builder::default();

    for item in vec.into_iter() {
        if !item.is_empty() {
            builder.append(item);
            builder.append(" ");
        }
    }

    builder.string().unwrap()
}

pub fn right() -> String {
    if CONFIG.venv_right_side {
        section_venv()
    } else {
        String::from("")
    }
}
