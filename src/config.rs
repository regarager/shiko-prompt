use crate::config_loader::load_config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Colors {
    pub cwd: String,
    pub git_branch: String,
    pub git_changes: String,
    pub arrow: String,
    pub venv: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub cwd_darken: f64,
    pub cwd_bold_last: bool,
    pub colors: Colors,
}

lazy_static! {
    pub static ref CONFIG: Config = load_config().expect("failed to load config");
}
