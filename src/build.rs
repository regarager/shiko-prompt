use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::RwLock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub debug_mode: bool,
}
