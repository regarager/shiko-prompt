# shiko-prompt

## Screenshots

### Default Theme

![Normal](./images/default.png)
![With VCS](./images/default_vcs.png)
![Python venv](./images/default_venv.png)

### Duskfox Theme

![Normal](./images/duskfox.png)
![With VCS](./images/duskfox_vcs.png)
![Python venv](./images/duskfox_venv.png)


### Campfire Theme
![Normal](./images/campfire.png)
![With VCS](./images/campfire_vcs.png)
![Python venv](./images/campfire_venv.png)

## Development Dependencies
Install the following dependencies: `rust`
Ex: `sudo pacman -Syu rust` (Arch)

## Installation
1. Clone the repository - `git clone https://github.com/regarager/shiko-prompt`
2. Move repository into home-folder (or other preferred directory) - `mv shiko-prompt ~`
3. Build the prompt binary - `cargo build --release` (alternatively, `SHIKO_THEME=./themes/<theme>.ron cargo build --release` for a different theme)
4. Add `source ~/shiko-prompt/shiko.zsh` to your `~/.zshrc` and reload it with `source ~/.zshrc`.
5. Enjoy!

## Customization

As of now, there is no configuration wizard, but you may change colors in `config.ron` and rebuilding the `shiko-prompt` binary.
