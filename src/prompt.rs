use crate::{
    config::CONFIG,
    cwd::section_cwd,
    util::{RESET, fg},
    vcs::section_git,
    venv::section_venv,
};
use string_builder::Builder;

pub fn left() -> String {
    let venv_text = if CONFIG.venv_right_side {
        String::from("")
    } else {
        section_venv()
    };

    let mut builder = Builder::default();

    builder.append(section_cwd());
    builder.append(section_git());
    builder.append(venv_text.clone());
    builder.append(fg(CONFIG.color3));
    builder.append(CONFIG.icon_arrow);
    builder.append(RESET);
    builder.append(" ");

    builder.string().unwrap()
}

pub fn right() -> String {
    if CONFIG.venv_right_side {
        section_venv()
    } else {
        String::from("")
    }
}
