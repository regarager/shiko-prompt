use crate::{config::CONFIG, icons, util::fg};
use regex::Regex;
use std::process::Command;

#[derive(Debug, Default)]
struct GitInfo {
    pub branch: String,
    pub ahead: usize,
    pub behind: usize,
    pub untracked: usize,
    pub unstaged: usize,
    pub staged: usize,
}

fn parse_branch() -> Option<String> {
    Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn parse_remote(line: &str) -> (usize, usize) {
    let ahead_match = Regex::new("ahead [0-9]+").unwrap().find(line);
    let behind_match = Regex::new("behind [0-9]+").unwrap().find(line);

    let ahead = match ahead_match {
        // remove 6 characters for "behind "
        Some(x) => line[x.range()][6..].parse::<usize>().unwrap_or(0),
        None => 0,
    };

    let behind = match behind_match {
        // remove 7 characters for "behind "
        Some(x) => line[x.range()][7..].parse::<usize>().unwrap_or(0),
        None => 0,
    };

    (ahead, behind)
}

fn construct_info() -> Option<GitInfo> {
    let output = Command::new("git").arg("status").arg("-sb").output().ok()?;

    if !output.status.success() {
        return None;
    }

    let branch = parse_branch()?;

    let mut info = GitInfo {
        branch,
        ahead: 0,
        behind: 0,
        untracked: 0,
        unstaged: 0,
        staged: 0,
    };

    let status = String::from_utf8(output.stdout).ok()?;

    for line in status.lines() {
        if line.len() < 2 {
            continue;
        }

        if line.starts_with("##") {
            (info.ahead, info.behind) = parse_remote(line);
        }

        match &line[0..2] {
            "??" => info.untracked += 1,
            s if s.starts_with(' ') || s.ends_with('M') => info.unstaged += 1,
            s if s.ends_with(' ') => info.staged += 1, // not 100% sure if this is correct
            _ => (),
        }
    }

    Some(info)
}

pub fn section_git() -> String {
    let Some(info) = construct_info() else {
        return String::new();
    };

    let main = format!("{}{} {}", fg(CONFIG.color2), icons::ICON_VCS_BRANCH, info.branch,);

    let changes = [
        (info.ahead, icons::ICON_VCS_AHEAD),
        (info.behind, icons::ICON_VCS_BEHIND),
        (info.staged, icons::ICON_VCS_STAGED),
        (info.unstaged, icons::ICON_VCS_UNSTAGED),
        (info.untracked, icons::ICON_VCS_UNTRACKED),
    ]
    .iter()
    .filter(|(count, _)| *count > 0)
    .map(|(count, icon)| format!("{count}{icon}"))
    .collect::<Vec<String>>()
    .join(" ");

    if changes.is_empty() {
        main
    } else {
        format!("{main} {}{changes}", fg(CONFIG.color_vcs_change))
    }
}
