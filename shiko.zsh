autoload -Uz vcs_info
autoload -Uz add-zsh-hook

build_prompt() {
  PROMPT=$(~/shiko-prompt/target/release/shiko --left)
  RPROMPT=$(~/shiko-prompt/target/release/shiko --right)
}

setopt prompt_subst

add-zsh-hook precmd build_prompt
add-zsh-hook chpwd build_prompt

build_prompt
