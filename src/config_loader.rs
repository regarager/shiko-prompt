use crate::config::Config;
use serde_json::Error;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

use generated::CONFIG_SOURCE;

pub fn load_config() -> Result<Config, Error> {
    serde_json::from_str(CONFIG_SOURCE)
}
