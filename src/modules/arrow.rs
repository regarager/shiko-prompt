use crate::{config::CONFIG, icons, utils::fg};

pub fn section_arrow() -> Option<String> {
    Some(format!("{}{}", fg(&CONFIG.colors.arrow), icons::ARROW))
}
