
source ~/shiko-prompt/colors.zsh
source ~/shiko-prompt/icons.zsh

autoload -Uz vcs_info

zstyle ":vcs_info:*" enable git svn
zstyle ":vcs_info:*" check-for-changes true

git_info() {
  local ref
  ref=$(git symbolic-ref HEAD 2> /dev/null) || \
  ref=$(git rev-parse --short HEAD 2> /dev/null) || return 0

  local branch="${ref#refs/heads/}"

  local changes=$(git status --porcelain | awk '{print substr($0, 1, 2)}' | sort | uniq -c | sed -e 's/^[[:space:]]*//')

  local change_status=""

  while IFS= read -r line; do
    count=$(echo $line | awk '{print $1}')
    code=${line: -2}

    case "$code" in
      "??") untracked=$((untracked + count)) ;;
      " M") modified_not_staged=$((modified_not_staged + count)) ;;
    esac
  done <<< "$changes"

  # Construct the output string
  if [[ $untracked -gt 0 ]]; then
    change_status+="+$untracked "
  fi
  if [[ $modified_not_staged -gt 0 ]]; then
    change_status+="*$modified_not_staged "
  fi

  local res="$ICON_BRANCH $branch "

  if [[ -n change_status ]]; then
    res+="$(text $COLOR_CHANGE)$change_status$reset "
  fi

  echo $res
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
  PROMPT=$(build_prompt)
}

chpwd() {
  update_vcs_info
}

precmd() {
  update_vcs_info
}

local reset="%f%k"

build_prompt() {
  local p=""

  p+=$(text $COLOR1)
  p+="%B%~%b "
  p+=$reset
  p+="$(text $COLOR2)"

  p+="$(git_info)"

  p+=$reset
  p+="$(text $COLOR4)"
  p+=$ICON_ARROW
  p+=$reset

  echo "$p "
}

setopt prompt_subst
PROMPT="$(build_prompt)"
