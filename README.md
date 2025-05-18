# shiko-prompt

## Screenshots

### Without VCS information
![Without VCS](./images/without_vcs.png)

### With VCS information
![With VCS](./images/with_vcs.png)

## Development Dependencies
Install the following dependencies: go git`
Ex: `sudo pacman -Syu go git` (Arch)

## Installation
1. Clone the repository - `git clone https://github.com/regarager/shiko-prompt`
2. Move repository into home-folder (or other preferred directory) - `mv shiko-prompt ~`
3. Build the prompt binary - `go build`
4. Add `source ~/shiko-prompt/shiko.zsh` to your `~/.zshrc` and reload it with `source ~/.zshrc`.
5. Enjoy!

## Customization

As of now, there is no configuration wizard, but you may change colors in `config.go` and rebuilding the `shiko-prompt` binary.
