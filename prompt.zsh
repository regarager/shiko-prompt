setopt prompt_subst

source ~/zsh-prompt/colors.zsh

autoload -Uz vcs_info
zstyle ":vcs_info:*" enable git svn
zstyle ":vcs_info:*" check-for-changes true

count_changes() {
  local unstaged=0
  local staged=0

  unstaged=$(git status --porcelain | grep -E "^.[MDA]" | wc -l | tr -d " ")


  staged=$(git status --porcelain | grep -E "^[MDA]" | wc -l | tr -d " ")


  echo "+${staged:+$staged} *${unstaged}"
}

zstyle ":vcs_info:*" formats "[%b]"
zstyle ":vcs_info:*" actionformats "[%b|%a]"

vcs_setup() {
  vcs_info
  if [[ -n ${vcs_info_msg_0_} ]]; then
    # Get the number of changes
    # Append the changes to the VCS info
    changes=$(count_changes)
    vcs_info_msg_0_="${vcs_info_msg_0_} $(text $COLOR3)$changes"
  fi
}
# Update vcs_info before each prompt
precmd() {
  vcs_setup
}

text_bg() {
  echo "%{%F{$1}%K{$2}%}"
}

text() {
  echo "%F{$1}"
}

bg() {
  echo "%K{$1}"
}

update_vcs_info() {
  vcs_setup

  PROMPT=$(build_prompt)
}

chpwd() {
  update_vcs_info
}

precmd() {
  update_vcs_info
}

local char_open="\ue0b6"
local char_close="\ue0b4"
local char_close_inner=""
local arrow="➔"
local reset="%f%k"

build_prompt() {
  local p=""

  p+=$(text $COLOR1)
  p+=" %B%~ %b"
  p+=$reset
  p+="$(text $COLOR2)"

  if [[ $vcs_info_msg_0_ = *[!\ ]* ]]; then
    p+='${vcs_info_msg_0_} '
  else
  fi

  p+=$reset
  p+="$(text $COLOR4)"
  p+=$arrow
  p+=$reset

  echo "$p "
}

PROMPT="$(build_prompt)"
