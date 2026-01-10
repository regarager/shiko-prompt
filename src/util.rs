use crate::config::{Color, ModuleConfig};

pub const RESET: &str = "%f%k";

pub fn module_fmt(config: &ModuleConfig) -> String {
    format!("{}{}", fg(&config.fg), bg(&config.bg))
}

pub fn fg(color: &Color) -> String {
    format!("%F{{{}}}", color)
}

pub fn bg(color: &Option<Color>) -> String {
    match color {
        Some(c) => format!("%K{{{}}}", c),
        None => String::new(),
    }
}

pub fn bold(text: &str) -> String {
    format!("%B{text}%b")
}

fn parse_hex(s: &str) -> f64 {
    u8::from_str_radix(s, 16).unwrap() as f64
}

pub fn darken(color: &Color, factor: f64) -> Color {
    match color {
        Color::Hex(c) => darken_hex(c.to_string(), factor),
        _ => color.clone(),
    }
}

fn darken_hex(color: String, factor: f64) -> Color {
    let r = parse_hex(&color[1..3]);
    let g = parse_hex(&color[3..5]);
    let b = parse_hex(&color[5..7]);

    Color::Hex(format!(
        "#{:02x}{:02x}{:02x}",
        (r * (1.0 - factor)) as u8,
        (g * (1.0 - factor)) as u8,
        (b * (1.0 - factor)) as u8
    ))
}
