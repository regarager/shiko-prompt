use lazy_static::lazy_static;
use regex::Regex;
use std::process::Command;

use crate::{config::CONFIG, utils::fg};

struct GitInfo {
    pub ahead: usize,
    pub behind: usize,
    pub untracked: usize,
    pub unstaged: usize,
    pub staged: usize,
}

lazy_static! {
    static ref REMOTE_REGEX: Regex = Regex::new("(ahead|behind) (\\d+)").unwrap();
}

fn parse_remote(line: &str) -> (usize, usize) {
    let mut ahead = 0;
    let mut behind = 0;

    for caps in REMOTE_REGEX.captures_iter(line) {
        let count: usize = caps[2].parse().unwrap_or(0);
        match &caps[1] {
            "ahead" => ahead = count,
            "behind" => behind = count,
            _ => {}
        }
    }

    (ahead, behind)
}

fn construct_info() -> Option<GitInfo> {
    let output = Command::new("git").arg("status").arg("-sb").output().ok()?;

    if !output.status.success() {
        return None;
    }

    let mut info = GitInfo {
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

pub fn section_git_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output()
        .ok();

    if let Some(o) = output
        && !o.stdout.is_empty()
    {
        Some(format!(
            "{}{} {}",
            fg(&CONFIG.colors.git_branch),
            &CONFIG.icons.git_branch,
            String::from_utf8(o.stdout).unwrap().trim_end()
        ))
    } else {
        None
    }
}

pub fn section_git_changes() -> Option<String> {
    let info = construct_info()?;
    let changes = [
        (info.ahead, &CONFIG.icons.git_ahead),
        (info.behind, &CONFIG.icons.git_behind),
        (info.staged, &CONFIG.icons.git_staged),
        (info.unstaged, &CONFIG.icons.git_unstaged),
        (info.untracked, &CONFIG.icons.git_untracked),
    ]
    .iter()
    .filter(|(count, _)| *count > 0)
    .map(|(count, icon)| format!("{count}{icon}"))
    .collect::<Vec<String>>()
    .join(" ");

    if changes.is_empty() {
        None
    } else {
        Some(format!("{}{changes}", fg(&CONFIG.colors.git_changes)))
    }
}
