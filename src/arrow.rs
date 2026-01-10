use crate::config::CONFIG;
use crate::icons;
use crate::util::{RESET, module_fmt};

pub fn section_arrow() -> Option<String> {
    Some(format!(
        "{}{}{}",
        module_fmt(&CONFIG.modules.arrow),
        icons::ARROW,
        RESET
    ))
}
