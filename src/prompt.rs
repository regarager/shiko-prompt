use crate::config::{CONFIG, ModuleConfig};
use crate::modules::arrow::section_arrow;
use crate::modules::cwd::section_cwd;
use crate::modules::vcs::{section_vcs_branch, section_vcs_changes};
use crate::modules::venv::section_venv;
use crate::utils::text::{RESET, bg, fg, fg_opt};

fn left() -> String {
    let mut vec: Vec<Option<String>> = Vec::new();

    vec.push(section_cwd());
    vec.push(section_vcs_branch());
    vec.push(section_vcs_changes());
    vec.push(if !CONFIG.venv_right_side {
        section_venv()
    } else {
        None
    });
    vec.push(section_arrow());

    let module_cfg = &CONFIG.modules;

    let modules = vec![
        Some(&module_cfg.directory),
        Some(&module_cfg.vcs_branch),
        Some(&module_cfg.vcs_changes),
        if !CONFIG.venv_right_side {
            Some(&module_cfg.venv)
        } else {
            None
        },
        Some(&module_cfg.arrow),
    ];

    let sections: Vec<(String, &ModuleConfig)> = vec
        .into_iter()
        .zip(modules)
        .filter_map(|(x, y)| {
            if let (Some(a), Some(b)) = (x, y)
                && b.enabled
            {
                Some((a, b))
            } else {
                None
            }
        })
        .collect();

    /*
     * for now, take bg separate from prefix, suffix
     * set prefix, suffix to part of the section
     */

    if sections.is_empty() {
        return String::new();
    }

    let mut res = String::new();

    res.push_str(&fg_opt(&sections[0].1.bg));
    if sections[0].1.bg.is_some() {
        res.push_str(&CONFIG.background_icons.0);
    }
    res.push_str(&fg(&sections[0].1.fg));
    res.push_str(&bg(&sections[0].1.bg));
    res.push_str(&sections[0].1.prefix);
    res.push_str(&sections[0].0);
    res.push_str(&sections[0].1.suffix);

    for i in 1..sections.len() {
        res.push_str(RESET);
        res.push_str(&bg(&sections[i].1.bg));

        if let Some(prev_bg) = &sections[i - 1].1.bg {
            res.push_str(&fg(prev_bg));
            res.push_str(&CONFIG.background_icons.1);
        }

        res.push_str(&fg(&sections[i].1.fg));
        res.push_str(&sections[i].1.prefix);
        res.push_str(&sections[i].0);
        res.push_str(&sections[i].1.suffix);
    }

    if let Some(bg) = &sections[sections.len() - 1].1.bg {
        res.push_str(RESET);
        res.push_str(&fg(bg));
        res.push_str(&CONFIG.background_icons.1);
        res.push(' ');
    }

    res.push_str(RESET);

    res
}

fn right() -> String {
    if let Some(venv) = section_venv()
        && CONFIG.venv_right_side
    {
        venv
    } else {
        String::new()
    }
}

pub fn print_left() {
    println!("{}", left());
}

pub fn print_right() {
    println!("{}", right());
}
