use std::{
    io,
    process::{Command, Output},
};

use crate::utils::icons;

#[derive(Debug, Default)]
struct GitInfo {
    pub ahead: usize,
    pub behind: usize,
    pub untracked: usize,
    pub unstaged: usize,
    pub staged: usize,
}

fn git(args: &[&str]) -> io::Result<Output> {
    Command::new("git").args(args).output()
}

// counts lines of git command
fn git_lines(args: &[&str]) -> usize {
    git(args)
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.lines().count())
        .unwrap_or(0)
}

fn is_repo() -> bool {
    git(&["rev-parse", "--is-inside-work-tree"])
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.trim() == "true")
        .unwrap_or(false)
}

fn get_ahead_behind() -> Option<(usize, usize)> {
    let output = git(&["rev-list", "--left-right", "--count", "HEAD...@{upstream}"]).ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let mut parts = stdout.split_whitespace();

    let ahead = parts.next()?.parse::<usize>().ok()?;
    let behind = parts.next()?.parse::<usize>().ok()?;

    Some((ahead, behind))
}

fn construct_info() -> Option<GitInfo> {
    if !is_repo() {
        return None;
    }

    let (ahead, behind) = get_ahead_behind().unwrap_or((0, 0));

    Some(GitInfo {
        ahead,
        behind,
        untracked: git_lines(&["ls-files", "--others", "--exclude-standard"]),
        unstaged: git_lines(&["diff", "--name-only"]),
        staged: git_lines(&["diff", "--cached", "--name-only"]),
    })
}

fn get_branch() -> Option<String> {
    if let Ok(output) = git(&["symbolic-ref", "--short", "HEAD"])
        && output.status.success()
        && !output.stdout.is_empty()
    {
        return String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim_end().to_string());
    }

    if let Ok(output) = git(&["rev-parse", "--short", "HEAD"])
        && output.status.success()
        && !output.stdout.is_empty()
    {
        return String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim_end().to_string());
    }

    None
}

pub fn section_vcs_branch() -> Option<String> {
    get_branch().map(|branch| format!("{} {}", icons::VCS_BRANCH, branch))
}

pub fn section_vcs_changes() -> Option<String> {
    let info = construct_info()?;
    let changes = [
        (info.ahead, icons::VCS_AHEAD),
        (info.behind, icons::VCS_BEHIND),
        (info.staged, icons::VCS_STAGED),
        (info.unstaged, icons::VCS_UNSTAGED),
        (info.untracked, icons::VCS_UNTRACKED),
    ]
    .iter()
    .filter(|(count, _)| *count > 0)
    .map(|(count, icon)| format!("{count}{icon}"))
    .collect::<Vec<String>>()
    .join(" ");

    if changes.is_empty() {
        None
    } else {
        Some(changes)
    }
}
