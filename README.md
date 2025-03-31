# shiko-prompt

## Screenshots

### Without VCS information
![Without VCS](./images/without_vcs.png)

### With VCS information
![With VCS](./images/with_vcs.png)

## Dependencies
Install the following dependencies: `python git sed`
Ex: `sudo pacman -Syu python git sed` (Arch Linux)

## Installation
1. Clone the repository - `git clone https://github.com/regarager/shiko-prompt`
2. Move repository into home-folder (or other preferred directory) - `mv shiko-prompt ~`
3. **If you would like to use the prebuilt configuration**, simply add `source ~/shiko-prompt/shiko.zsh` to your `~/.zshrc` and reload it.
4. **Otherwise**, Run the setup script - `python setup.py`
5. Add `source ~/shiko-prompt/prompt.zsh` to your `~/.zshrc`
6. Reload `~/.zshrc` - `source ~/.zshrc`
7. Enjoy!

## Customization
Customization may be done with the setup script - `python setup.py`
