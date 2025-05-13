COLOR1=#2bd4ff
COLOR2=#00e600
COLOR3=#b5fd0d
COLOR_VCS_CHANGE=#f4d03f

ICON_LEFT=""
ICON_RIGHT=""
ICON_ARROW="➔"
ICON_VCS_BRANCH=""
ICON_VCS_AHEAD=""
ICON_VCS_BEHIND=""

autoload -Uz vcs_info
autoload -Uz add-zsh-hook

zstyle ":vcs_info:*" enable git svn
zstyle ":vcs_info:*" check-for-changes true

git_info() {
  local ref
  ref=$(git symbolic-ref HEAD 2> /dev/null) || \
  ref=$(git rev-parse --short HEAD 2> /dev/null) || return 0

  local branch="${ref#refs/heads/}"

  local git_status=$(git status -sb)

  local remote_changes=$(echo $git_status | head -n 1)

  ahead=0
  behind=0
  [[ $remote_changes =~ ahead\ ([0-9]+) ]]; ahead=${match[1]}
  [[ $remote_changes =~ behind\ ([0-9]+) ]]; behind=${match[1]}

  local changes=$(echo $git_status | tail -n +2 | awk '{print substr($0, 1, 2)}' | sort | uniq -c | sed -e 's/^[[:space:]]*//')

  local change_status=""

  while IFS= read -r line; do
    count=$(echo $line | awk '{print $1}')
    code=${line: -2}

    case "$code" in
      "??") untracked=$((untracked + count)) ;;
      " "*) modified_not_staged=$((modified_not_staged + count)) ;;
      *"M") modified_not_staged=$((modified_not_staged + count)) ;;
      *" ") modified_staged=$((modified_staged + count)) ;;

    esac
  done <<< "$changes"

  if [[ $untracked -gt 0 ]]; then
    change_status+="$untracked? "
  fi
  if [[ $modified_not_staged -gt 0 ]]; then
    change_status+="$modified_not_staged* "
  fi
  if [[ $modified_staged -gt 0 ]]; then
    change_status+="$modified_staged+ "
  fi
  if [[ $ahead -gt 0 ]]; then
    change_status+="$ICON_VCS_AHEAD$ahead "
  fi
  if [[ $behind -gt 0 ]]; then
    change_status+="$ICON_VCS_BEHIND$behind"
  fi

  local res="$ICON_VCS_BRANCH $branch "

  if [[ -n $(echo $change_status | xargs) ]]; then
    res+="$(text $COLOR_VCS_CHANGE)$change_status$reset"
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

local reset="%f%k"

build_prompt() {
  local p=""

  p+=$(text $COLOR1)
  p+="%B%~%b "
  p+=$reset
  p+="$(text $COLOR2)"

  p+="$(git_info)"

  p+=$reset
  p+="$(text $COLOR3)"
  p+=$ICON_ARROW
  p+=$reset

  PROMPT="$p "
}

setopt prompt_subst

add-zsh-hook precmd build_prompt
add-zsh-hook chpwd build_prompt

build_prompt
