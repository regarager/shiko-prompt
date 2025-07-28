# shiko-prompt

## Screenshots

### Without VCS information
![Without VCS](./images/without_vcs.png)

### With VCS information
![With VCS](./images/with_vcs.png)

## Development Dependencies
Install the following dependencies: `rust`
Ex: `sudo pacman -Syu rust` (Arch)

## Installation
1. Clone the repository - `git clone https://github.com/regarager/shiko-prompt`
2. Move repository into home-folder (or other preferred directory) - `mv shiko-prompt ~`
3. Copy a theme from `themes/` into the project root as `config.ron`
4. Build the prompt binary - `cargo build --release`
5. Add `source ~/shiko-prompt/shiko.zsh` to your `~/.zshrc` and reload it with `source ~/.zshrc`.
6. Enjoy!

## Customization

As of now, there is no configuration wizard, but you may change colors in `config.ron` and rebuilding the `shiko-prompt` binary.
