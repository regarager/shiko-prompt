pub const RESET: &str = "%f%k";

pub fn fg(color: &str) -> String {
    format!("%F{{{color}}}")
}

pub fn bg(color: &str) -> String {
    format!("%K{{{color}}}")
}

pub fn bold(text: &str) -> String {
    format!("%B{text}%b")
}

fn parse_hex(s: &str) -> f64 {
    u8::from_str_radix(s, 16).unwrap() as f64
}

pub fn darken(color: &str, factor: f64) -> String {
    let r = parse_hex(&color[1..3]);
    let g = parse_hex(&color[3..5]);
    let b = parse_hex(&color[5..7]);

    format!(
        "#{:02x}{:02x}{:02x}",
        (r * (1.0 - factor)) as u8,
        (g * (1.0 - factor)) as u8,
        (b * (1.0 - factor)) as u8
    )
}
