use crate::{
    config::CONFIG,
    cwd::section_cwd,
    util::{RESET, fg},
    vcs::section_git,
    venv::section_venv,
};

pub fn left() -> String {
    let venv_text = if CONFIG.venv_right_side {
        String::from("")
    } else {
        let text = section_venv();
        if text.is_empty() {
            String::from("")
        } else {
            format!("{} ", section_venv())
        }
    };

    format!(
        "{}{}{}{}{}{} ",
        section_cwd(),
        section_git(),
        venv_text,
        fg(CONFIG.color3),
        CONFIG.icon_arrow,
        RESET
    )
}

pub fn right() -> String {
    if CONFIG.venv_right_side {
        section_venv()
    } else {
        String::from("")
    }
}
