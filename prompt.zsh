setopt prompt_subst

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
    changes=$(count_changes)
    # Append the changes to the VCS info
    vcs_info_msg_0_="${vcs_info_msg_0_} $changes"
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

COLOR1=#05668d
COLOR2=#028090
COLOR3=#00a896
COLOR4=#02c39a
COLOR5=#f0f3bd

COLOR_BG=$COLOR1
COLOR_TEXT=#ffffff

local char_open="\ue0b6"
local char_close="\ue0b4"
local char_close_inner=""
local reset="%f%k"

build_prompt() {
  local p=""

  p+=$(text $COLOR1)
  p+=$char_open
  p+=$reset
  p+="$(text_bg $COLOR_TEXT $COLOR1)"
  p+="%~ "
  p+=$reset
  p+="$(text $COLOR1)"

  if [[ $vcs_info_msg_0_ = *[!\ ]* ]]; then
    p+="$(bg $COLOR3)"
    p+=$char_close_inner
    p+=$reset
    p+="$(text_bg $COLOR_TEXT $COLOR3) "
    p+='${vcs_info_msg_0_} '
    p+=$reset
    p+="$(text $COLOR3)"
    p+=$char_close
  else
    p+=$char_close
  fi

  p+=$reset

  echo "$p "
}

PROMPT="$(build_prompt)"
