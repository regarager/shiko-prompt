use crate::config::Config;
use serde_json::Error;
use std::fs;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

use generated::CONFIG_SOURCE;

#[allow(dead_code)]
pub enum ConfigSource {
    Hardcoded(&'static str),
    Hotload(&'static str),
}

pub fn load_config() -> Result<Config, Error> {
    match CONFIG_SOURCE {
        ConfigSource::Hardcoded(s) => serde_json::from_str(s),
        ConfigSource::Hotload(s) => {
            serde_json::from_slice(&fs::read(s).expect("failed to read config"))
        }
    }
}
