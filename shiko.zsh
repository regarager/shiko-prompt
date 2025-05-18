autoload -Uz vcs_info
autoload -Uz add-zsh-hook

build_prompt() {
  PROMPT=$(~/shiko-prompt/shiko-prompt)
}

setopt prompt_subst

add-zsh-hook precmd build_prompt
add-zsh-hook chpwd build_prompt

build_prompt
