use crate::{
    arrow::section_arrow, config::CONFIG, cwd::section_cwd, vcs::section_vcs_branch,
    vcs::section_vcs_changes, venv::section_venv,
};
use string_builder::Builder;

pub fn left() -> String {
    let mut vec: Vec<Option<String>> = Vec::new();

    vec.push(section_cwd());
    vec.push(section_vcs_branch());
    vec.push(section_vcs_changes());
    if !CONFIG.venv_right_side {
        vec.push(section_venv());
    }
    vec.push(section_arrow());

    let mut builder = Builder::default();

    for item in vec.into_iter().flatten() {
        builder.append(item);
        builder.append(" ");
    }

    builder.string().unwrap()
}

pub fn right() -> String {
    if let Some(venv) = section_venv()
        && CONFIG.venv_right_side
    {
        venv
    } else {
        String::new()
    }
}
