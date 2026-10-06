use crate::config_loader::load_config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Colors {
  pub cwd: String,
  pub git_branch: String,
  pub git_changes: String,
  pub arrow: String,
  pub venv: String,
  pub nixshell: String,
}

impl Default for Colors {
  fn default() -> Self {
    Self {
      cwd: "#2bd4ff".into(),
      git_branch: "#00e600".into(),
      git_changes: "#f4d03f".into(),
      arrow: "#b5fd0d".into(),
      venv: "#00c0a3".into(),
      nixshell: "#4d72ba".into(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Icons {
  pub arrow: String,
  pub git_ahead: String,
  pub git_behind: String,
  pub git_branch: String,
  pub git_staged: String,
  pub git_unstaged: String,
  pub git_untracked: String,
  pub venv: String,
  pub nixshell: String,
}

impl Default for Icons {
  fn default() -> Self {
    Self {
      arrow: "\u{f061}".into(),      // 
      git_ahead: "\u{f176}".into(),  // 
      git_behind: "\u{f175}".into(), // 
      git_branch: "\u{e0a0}".into(), // 
      git_staged: "+".into(),
      git_unstaged: "*".into(),
      git_untracked: "?".into(),
      venv: "\u{e73c}".into(),     // 
      nixshell: "\u{f313}".into(), // 
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
  pub cwd_darken: f64,
  pub cwd_bold_last: bool,
  pub colors: Colors,
  pub icons: Icons,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      cwd_darken: 0.25,
      cwd_bold_last: true,
      colors: Colors::default(),
      icons: Icons::default(),
    }
  }
}

lazy_static! {
  pub static ref CONFIG: Config = load_config().expect("failed to load config");
}
