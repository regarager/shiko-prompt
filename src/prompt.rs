use crate::{
    arrow::section_arrow, config::CONFIG, config::ModuleConfig, cwd::section_cwd,
    util::BACKGROUND_LEFT, util::BACKGROUND_RIGHT, util::RESET, util::bg, util::fg, util::fg_opt,
    vcs::section_vcs_branch, vcs::section_vcs_changes, venv::section_venv,
};
use string_builder::Builder;

pub fn left() -> String {
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

    let mut builder = Builder::default();

    let zip = vec
        .into_iter()
        .zip(modules.into_iter())
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

    /*
     * for now, take bg separate from prefix, suffix
     * set prefix, suffix to part of the section
     */

    if zip.is_empty() {
        return String::new();
    }

    builder.append(fg_opt(&zip[0].1.bg));
    if zip[0].1.bg.is_some() {
        builder.append(BACKGROUND_LEFT);
    }
    builder.append(fg(&zip[0].1.fg));
    builder.append(bg(&zip[0].1.bg));
    builder.append(zip[0].1.prefix.clone());
    builder.append(zip[0].0.clone());
    builder.append(zip[0].1.suffix.clone());

    for i in 1..zip.len() {
        builder.append(RESET);
        builder.append(bg(&zip[i].1.bg));

        if let Some(prev_bg) = &zip[i - 1].1.bg {
            builder.append(fg(&prev_bg));
            builder.append(BACKGROUND_RIGHT);
        }

        builder.append(fg(&zip[i].1.fg));
        builder.append(zip[i].1.prefix.clone());
        builder.append(zip[i].0.clone());
        builder.append(zip[i].1.suffix.clone());
    }

    if let Some(bg) = &zip[zip.len() - 1].1.bg {
        builder.append(RESET);
        builder.append(fg(bg));
        builder.append(BACKGROUND_RIGHT);
        builder.append(" ");
    }

    builder.append(RESET);

    builder.string().unwrap()
}

pub fn right() -> String {
    if let Some(venv) = section_venv()
        && CONFIG.venv_right_side
    {
        venv
    } else {
        String::new()
    }
}
