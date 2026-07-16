pub fn zsh_init() {
    println!(
        r#"
autoload -Uz add-zsh-hook

build_prompt() {{
  PROMPT=$(shiko left)
  RPROMPT=$(shiko right)
}}

setopt prompt_subst

add-zsh-hook precmd build_prompt
add-zsh-hook chpwd build_prompt

build_prompt
        "#
    );
}
