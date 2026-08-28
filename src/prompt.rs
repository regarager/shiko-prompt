use crate::modules::arrow::section_arrow;
use crate::modules::cwd::section_cwd;
use crate::modules::git::{section_git_branch, section_git_changes};
use crate::modules::venv::section_venv;
use crate::utils::RESET;

fn left() -> String {
    let vec: Vec<Option<String>> = vec![
        section_cwd(),
        section_git_branch(),
        section_git_changes(),
        section_arrow(),
    ];

    let mut res = String::new();

    vec.into_iter().flatten().for_each(|it| {
        res.push_str(&it);
        res.push(' ');
    });

    res.push_str(RESET);

    res
}

fn right() -> String {
    section_venv().unwrap_or_default()
}

pub fn print_left() {
    println!("{}", left());
}

pub fn print_right() {
    println!("{}", right());
}
