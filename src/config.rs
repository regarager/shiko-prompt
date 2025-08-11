use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // colors
    #[serde(default = "default_color1")]
    pub color1: &'static str,
    #[serde(default = "default_color2")]
    pub color2: &'static str,
    #[serde(default = "default_color3")]
    pub color3: &'static str,
    #[serde(default = "default_color_vcs_change")]
    pub color_vcs_change: &'static str,
    #[serde(default = "default_color_venv")]
    pub color_venv: &'static str,

    // icons
    #[serde(default = "default_icon_arrow")]
    pub icon_arrow: &'static str,
    #[serde(default = "default_icon_section_left")]
    pub icon_section_left: &'static str,
    #[serde(default = "default_icon_section_right")]
    pub icon_section_right: &'static str,
    #[serde(default = "default_icon_vcs_ahead")]
    pub icon_vcs_ahead: &'static str,
    #[serde(default = "default_icon_vcs_behind")]
    pub icon_vcs_behind: &'static str,
    #[serde(default = "default_icon_vcs_branch")]
    pub icon_vcs_branch: &'static str,
    #[serde(default = "default_icon_vcs_staged")]
    pub icon_vcs_staged: &'static str,
    #[serde(default = "default_icon_vcs_unstaged")]
    pub icon_vcs_unstaged: &'static str,
    #[serde(default = "default_icon_vcs_untracked")]
    pub icon_vcs_untracked: &'static str,
    #[serde(default = "default_icon_venv")]
    pub icon_venv: &'static str,

    // misc
    #[serde(default = "default_cwd_darken")]
    pub cwd_darken: bool,
    #[serde(default = "default_cwd_darken_factor")]
    pub cwd_darken_factor: f64,
    #[serde(default = "default_cwd_highlight_last")]
    pub cwd_highlight_last: bool,
    #[serde(default = "default_venv_right_side")]
    pub venv_right_side: bool,
}

fn default_color1() -> &'static str {
    "#2bd4ff"
}

fn default_color2() -> &'static str {
    "#00e600"
}

fn default_color3() -> &'static str {
    "#b5fd0d"
}

fn default_color_vcs_change() -> &'static str {
    "#f4d03f"
}

fn default_color_venv() -> &'static str {
    "#00c0a3"
}

fn default_icon_arrow() -> &'static str {
    "➔"
}

fn default_icon_section_left() -> &'static str {
    ""
}

fn default_icon_section_right() -> &'static str {
    ""
}

fn default_icon_vcs_ahead() -> &'static str {
    ""
}

fn default_icon_vcs_behind() -> &'static str {
    ""
}

fn default_icon_vcs_branch() -> &'static str {
    ""
}

fn default_icon_vcs_staged() -> &'static str {
    "+"
}

fn default_icon_vcs_unstaged() -> &'static str {
    "*"
}

fn default_icon_vcs_untracked() -> &'static str {
    "?"
}

fn default_icon_venv() -> &'static str {
    ""
}

fn default_cwd_darken_factor() -> f64 {
    0.25
}

fn default_cwd_darken() -> bool {
    true
}

fn default_cwd_highlight_last() -> bool {
    true
}

fn default_venv_right_side() -> bool {
    true
}

lazy_static! {
    static ref CONFIG_TEXT: String = {
        let config_file =
            PathBuf::from(option_env!("SHIKO_THEME").unwrap_or("./themes/default.ron"))
                .canonicalize()
                .expect("config file not found");

        fs::read_to_string(config_file).expect("error while reading config file")
    };
    pub static ref CONFIG: Config =
        ron::from_str(&CONFIG_TEXT).expect("failed to parse config.ron");
}
