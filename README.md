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
3. Build the prompt binary - `./install.sh <theme>` (themes must be given as file paths, such as the ones listed in `themes/`)
4. Add `eval "$(shiko init)"` to your `~/.zshrc` and reload it with `source ~/.zshrc`.
5. Enjoy!

## Customization

As of now, there is no configuration wizard, but you may change colors in `config.ron` and rebuilding the `shiko-prompt` binary.
