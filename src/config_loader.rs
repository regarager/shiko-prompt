use crate::config::Config;
use generated::CONFIG_SOURCE;

mod generated {
  include!(concat!(env!("OUT_DIR"), "/config.rs"));
}

pub fn load_config() -> Result<Config, toml::de::Error> {
  toml::from_str(CONFIG_SOURCE)
}
