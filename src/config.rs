#[derive(Debug)]
pub struct Config {
    pub cwd_highlight_last: bool,
    pub cwd_darken: bool,
    pub cwd_darken_factor: f64,

    pub color1: &'static str,
    pub color2: &'static str,
    pub color3: &'static str,
    pub color_vcs_change: &'static str,

    pub icon_section_left: &'static str,
    pub icon_section_right: &'static str,
    pub icon_arrow: &'static str,
    pub icon_vcs_branch: &'static str,
    pub icon_vcs_ahead: &'static str,
    pub icon_vcs_behind: &'static str,
    pub icon_vcs_staged: &'static str,
    pub icon_vcs_unstaged: &'static str,
    pub icon_vcs_untracked: &'static str,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            cwd_highlight_last: true,
            cwd_darken: true,
            cwd_darken_factor: 0.25,

            // colors,
            color1: "#2bd4ff",
            color2: "#00e600",
            color3: "#b5fd0d",
            color_vcs_change: "#f4d03f",

            // icons,
            icon_section_left: "",
            icon_section_right: "",
            icon_arrow: "➔",
            icon_vcs_branch: "",
            icon_vcs_ahead: "",
            icon_vcs_behind: "",
            icon_vcs_staged: "+",
            icon_vcs_unstaged: "*",
            icon_vcs_untracked: "?",
        }
    }
}

pub const CONFIG: Config = Config::new();
