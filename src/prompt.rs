use crate::{
    arrow::section_arrow,
    config::CONFIG,
    cwd::section_cwd,
    vcs::section_vcs_branch,
    vcs::section_vcs_changes,
    venv::section_venv,
};
use string_builder::Builder;

pub fn left() -> String {
    let mut vec: Vec<String> = Vec::new();

    vec.push(section_cwd());
    vec.push(section_vcs_branch());
    vec.push(section_vcs_changes());
    if !CONFIG.venv_right_side {
        vec.push(section_venv());
    }
    vec.push(section_arrow());

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
