use crate::{config::CONFIG, utils::fg};

pub fn section_arrow() -> Option<String> {
    Some(format!(
        "{}{}",
        fg(&CONFIG.colors.arrow),
        &CONFIG.icons.arrow
    ))
}
