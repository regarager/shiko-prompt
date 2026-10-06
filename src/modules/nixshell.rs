use std::env;

use crate::config::CONFIG;
use crate::utils::fg;

pub fn section_nixshell() -> Option<String> {
  env::var("IN_NIX_SHELL")
    .ok()
    .map(|_| format!("{}{}", fg(&CONFIG.colors.nixshell), CONFIG.icons.nixshell))
}
