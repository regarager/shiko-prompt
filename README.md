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

As of now, there is no configuration wizard, but you may change colors in the theme JSON files in the `themes/` directory and rebuild the `shiko-prompt` binary.

### Theme Selection

To build with a specific theme, pass it as an argument to the install script:
```bash
./install.sh ./themes/campfire.json
```

Or build manually:
```bash
SHIKO_THEME=./themes/your-theme.json cargo build --release
```

The theme is compiled into the binary at build time. After building, add `eval "$(shiko init)"` to your `~/.zshrc`.
