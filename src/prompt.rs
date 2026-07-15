use crate::config::{CONFIG, ModuleConfig};
use crate::modules::arrow::section_arrow;
use crate::modules::cwd::section_cwd;
use crate::modules::vcs::{section_vcs_branch, section_vcs_changes};
use crate::modules::venv::section_venv;
use crate::utils::text::{RESET, fg};

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

    let zip = vec
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
        .collect::<Vec<(String, &ModuleConfig)>>();

    if zip.is_empty() {
        return String::new();
    }

    let mut res = String::new();

    res.push_str(&fg(&zip[0].1.color));
    res.push_str(&zip[0].0.clone());
    res.push_str(&zip[0].1.suffix.clone());

    for i in 1..zip.len() {
        res.push_str(RESET);
        res.push_str(&fg(&zip[i].1.color));
        res.push_str(&zip[i].0.clone());
        res.push_str(&zip[i].1.suffix.clone());
    }

    res.push_str(RESET);
    res.push_str(" ");

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