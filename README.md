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

The default theme configuration is shown below:

```json
{
  "cwd_darken": 0.25,
  "cwd_bold_last": true,
  "colors": {
    "cwd": "#ccb1ed",
    "git_branch": "#b1d196",
    "git_changes": "#f9cb8c",
    "venv": "#65b1cd",
    "arrow":"#f0a6cc"
  },
  "icons": {
    "arrow": "➔",
    "git_ahead": "",
    "git_behind": "",
    "git_branch": "",
    "git_staged": "+",
    "git_unstaged": "*",
    "git_untracked": "?",
    "venv": ""
  }
}
```

| Key | Type | Description |
|-----|------|-------------|
| `cwd_darken` | number | Opacity of the working directory prefix (0.0 = fully dimmed, 1.0 = full brightness) |
| `cwd_bold_last` | boolean | Whether the last path component in the cwd is rendered in bold |
| `colors.cwd` | hex color | Color of the working directory text |
| `colors.git_branch` | hex color | Color of the git branch name |
| `colors.git_changes` | hex color | Color of the git change indicators (staged/unstaged) |
| `colors.venv` | hex color | Color of the virtual environment prefix |
| `colors.arrow` | hex color | Color of the arrow separator between prompt segments |
| `icons.arrow` | string | Character used as the separator between prompt segments |
| `icons.git_ahead` | string | Character shown when the branch is ahead of the remote |
| `icons.git_behind` | string | Character shown when the branch is behind the remote |
| `icons.git_branch` | string | Character shown before the git branch name |
| `icons.git_staged` | string | Character shown for staged changes |
| `icons.git_unstaged` | string | Character shown for unstaged (modified) changes |
| `icons.git_untracked` | string | Character shown for untracked files |

All colors should be written in the format `#123456` (6 digit hexadecimal with leading #).
