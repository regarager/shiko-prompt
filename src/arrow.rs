use crate::util::{fg, RESET};
use crate::icons;
use crate::config::CONFIG;

pub fn section_arrow() -> Option<String> {
   Some(format!("{}{}{}", fg(&CONFIG.modules.arrow), icons::ICON_ARROW, RESET))
}
