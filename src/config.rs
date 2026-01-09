use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

use generated::CONFIG_TEXT;

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
    false
}

lazy_static! {
    pub static ref CONFIG: Config = ron::from_str(CONFIG_TEXT).expect("failed to parse config.ron");
}
