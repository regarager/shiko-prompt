use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fmt;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

use generated::CONFIG_TEXT;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Hex(String),
    Terminal(u8),
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Hex(s) => write!(f, "{}", s),
            Color::Terminal(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub fg: Color,
    pub bg: Option<Color>,
    #[serde(default)]
    pub prefix: String,
    #[serde(default = "suffix_default")]
    pub suffix: String,
    #[serde(default = "enabled")]
    pub enabled: bool,
}

fn enabled() -> bool {
    true
}

fn suffix_default() -> String {
    String::new()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modules {
    pub directory: ModuleConfig,
    pub vcs_branch: ModuleConfig,
    pub vcs_changes: ModuleConfig,
    pub venv: ModuleConfig,
    pub arrow: ModuleConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // misc
    #[serde(default = "default_background_icon")]
    pub background_icons: (String, String),
    #[serde(default = "default_cwd_darken")]
    pub cwd_darken: bool,
    #[serde(default = "default_cwd_darken_factor")]
    pub cwd_darken_factor: f64,
    #[serde(default = "default_cwd_highlight_last")]
    pub cwd_highlight_last: bool,
    #[serde(default = "default_venv_right_side")]
    pub venv_right_side: bool,

    pub modules: Modules,
}

fn default_background_icon() -> (String, String) {
    // , 
    (String::from("\u{e0b6}"), String::from("\u{e0b4}"))
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
    pub static ref CONFIG: Config =
        serde_json::from_str(CONFIG_TEXT).expect("failed to parse configuration");
}
