use crate::{
    config::CONFIG,
    util::{RESET, fg},
};
use regex::Regex;
use std::{error::Error, process::Command};
#[derive(Debug)]
struct GitInfo {
    pub branch: String,
    pub ahead: i32,
    pub behind: i32,
    pub untracked: i32,
    pub unstaged: i32,
    pub staged: i32,
}

fn parse_branch() -> Option<String> {
    if let Ok(output) = Command::new("git").arg("symbolic-ref").arg("HEAD").output() {
        if !output.stderr.is_empty() {
            return None;
        }
        let branch = String::from_utf8(output.stdout).unwrap();
        Some(
            branch
                .strip_prefix("refs/heads/")
                .unwrap_or(&branch)
                .trim()
                .to_string(),
        )
    } else {
        None
    }
}

fn parse_remote(line: &str) -> (i32, i32) {
    let ahead_match = Regex::new("ahead [0-9]+").unwrap().find(line);
    let behind_match = Regex::new("behind [0-9]+").unwrap().find(line);

    let ahead = match ahead_match {
        Some(x) => line[x.range()].parse::<i32>().unwrap_or(0),
        None => 0,
    };

    let behind = match behind_match {
        Some(x) => line[x.range()].parse::<i32>().unwrap_or(0),
        None => 0,
    };

    (ahead, behind)
}

fn construct_info() -> Option<GitInfo> {
    let output = Command::new("git").arg("status").arg("-sb").output();

    if output.is_err() {
        return None;
    }

    let branch = parse_branch();

    branch.as_ref()?;

    let git_status = output.unwrap();
    let status = String::from_utf8(git_status.stdout).unwrap();

    let mut ahead = 0;
    let mut behind = 0;
    let mut untracked = 0;
    let mut unstaged = 0;
    let mut staged = 0;

    for line in status.split("\n") {
        if line.len() < 2 {
            continue;
        }

        if line.starts_with("##") {
            (ahead, behind) = parse_remote(line);
        }

        let pref = &line[0..2];

        if pref == "??" {
            untracked += 1;
        } else if pref.starts_with(' ') || pref.chars().nth(1) == Some('M') {
            unstaged += 1;
        } else if pref.chars().nth(1) == Some(' ') {
            staged += 1;
        }
    }

    Some(GitInfo {
        branch: branch.unwrap(),
        ahead,
        behind,
        untracked,
        unstaged,
        staged,
    })
}

pub fn section_git() -> String {
    let res = construct_info();

    if res.is_none() {
        return String::from("");
    }

    let info = res.unwrap();

    let main = format!(
        "{RESET}{}{} {}{RESET}",
        fg(CONFIG.color2),
        CONFIG.icon_vcs_branch,
        info.branch,
    );

    let mut changes = String::from(" ");

    let ahead_str = format!("{}{} ", info.ahead, CONFIG.icon_vcs_ahead);
    let behind_str = format!("{}{} ", info.behind, CONFIG.icon_vcs_behind);
    let staged_str = format!("{}{} ", info.staged, CONFIG.icon_vcs_staged);
    let unstaged_str = format!("{}{} ", info.unstaged, CONFIG.icon_vcs_unstaged);
    let untracked_str = format!("{}{} ", info.untracked, CONFIG.icon_vcs_untracked);

    if info.ahead > 0 {
        changes.push_str(&ahead_str);
    }

    if info.behind > 0 {
        changes.push_str(&behind_str);
    }

    if info.staged > 0 {
        changes.push_str(&staged_str);
    }

    if info.unstaged > 0 {
        changes.push_str(&unstaged_str);
    }

    if info.untracked > 0 {
        changes.push_str(&untracked_str);
    }

    let color = fg(CONFIG.color_vcs_change);
    format!("{RESET}{main}{color}{changes}")
}
