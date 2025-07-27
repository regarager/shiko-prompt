use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // colors
    pub color1: &'static str,
    pub color2: &'static str,
    pub color3: &'static str,
    pub color_vcs_change: &'static str,
    pub color_venv: &'static str,

    // icons
    pub icon_arrow: &'static str,
    pub icon_section_left: &'static str,
    pub icon_section_right: &'static str,
    pub icon_vcs_ahead: &'static str,
    pub icon_vcs_behind: &'static str,
    pub icon_vcs_branch: &'static str,
    pub icon_vcs_staged: &'static str,
    pub icon_vcs_unstaged: &'static str,
    pub icon_vcs_untracked: &'static str,
    pub icon_venv: &'static str,

    // misc
    pub cwd_darken: bool,
    pub cwd_darken_factor: f64,
    pub cwd_highlight_last: bool,
    pub venv_right_side: bool,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            color1: "#2bd4ff",
            color2: "#00e600",
            color3: "#b5fd0d",
            color_vcs_change: "#f4d03f",
            color_venv: "#00c0a3",

            icon_arrow: "➔",
            icon_section_left: "",
            icon_section_right: "",
            icon_vcs_ahead: "",
            icon_vcs_behind: "",
            icon_vcs_branch: "",
            icon_vcs_staged: "+",
            icon_vcs_unstaged: "*",
            icon_vcs_untracked: "?",
            icon_venv: "",

            cwd_darken_factor: 0.25,
            cwd_darken: true,
            cwd_highlight_last: true,
            venv_right_side: true,
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_text = include_str!("../config.ron");
        ron::from_str(config_text).expect("failed to parse config.ron")
    };
}
