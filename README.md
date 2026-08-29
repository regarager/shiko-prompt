# shiko-prompt

An opinionated Rust-based zsh prompt builder built to be lightweight and minimal. If you want something that is quick and easy to configure, then this is the propmt for you.

![kanagawa](media/kanagawa.gif)

## Installation

Run `./install.sh <theme>` to build and install the prompt for a specific theme (e.g., `./install.sh themes/kanagawa.json`).

## Usage

Add `eval "$(shiko init)"` to your `.zshrc`.

## Customization
The default theme may be found at `themes/default.json`.

### Options
- `cwd_darken`: darken working directory prefix (0.0–1.0)
- `cwd_bold_last`: bold the last path component
- `colors.cwd`: working directory color
- `colors.git_branch`: git branch name color
- `colors.git_changes`: uncommitted changes color
- `colors.arrow`: arrow separator color
- `colors.venv`: virtual environment color

All `colors.*` values must be 6-digit hexadecimal (e.g., `#ffffff`).
