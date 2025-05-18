package main

import (
	"os/exec"
	"regexp"
	"strconv"
	"strings"
)

type GitInfo struct {
	Branch    string
	Ahead     int
	Behind    int
	Untracked int
	Unstaged  int
	Staged    int
}

func parse_branch() string {
	branch, err := exec.Command("git", "symbolic-ref", "HEAD").Output()

	if err != nil {
		return ""
	}

	return strings.TrimSuffix(strings.TrimPrefix(string(branch), "refs/heads/"), "\n")
}

func parse_remote(line string, git_info *GitInfo) {
	ahead_match := regexp.MustCompile("ahead [0-9]+").FindString(line)
	behind_match := regexp.MustCompile("behind [0-9]+").FindString(line)

	// hardcoded values for trimming
	if len(ahead_match) > 0 {
		git_info.Ahead, _ = strconv.Atoi(ahead_match[6:])
	}

	if len(behind_match) > 0 {
		git_info.Behind, _ = strconv.Atoi(behind_match[7:])
	}
}

func git_info() (GitInfo, error) {
	res := GitInfo{}

	output, err := exec.Command("git", "status", "-sb").Output()

	if err != nil {
		return res, err
	}

	res.Branch = parse_branch()

	status := string(output)

	for line := range strings.SplitSeq(status, "\n") {
		if len(line) < 2 {
			continue
		}

		if strings.HasPrefix(line, "##") {
			parse_remote(line, &res)
		}

		pref := line[:2]

		if pref == "??" {
			res.Untracked++
		} else if pref[0] == ' ' || pref[1] == 'M' {
			res.Unstaged++
		} else if pref[1] == ' ' {
			res.Staged++
		}
	}

	return res, nil
}

func section_git() string {
	var res strings.Builder

	info, err := git_info()

	if err != nil {
		return ""
	}

	res.WriteString(RESET)
	res.WriteString(text(COLOR2))
	res.WriteString(ICON_VCS_BRANCH)
	res.WriteString(" ")
	res.WriteString(info.Branch)
	res.WriteString(" ")

	res.WriteString(text(COLOR_VCS_CHANGE))

	if info.Ahead > 0 {
		res.WriteString(strconv.Itoa(info.Ahead))
		res.WriteString(ICON_VCS_AHEAD)
		res.WriteString(" ")
	}

	if info.Behind > 0 {
		res.WriteString(strconv.Itoa(info.Behind))
		res.WriteString(ICON_VCS_BEHIND)
		res.WriteString(" ")
	}

	if info.Staged > 0 {
		res.WriteString(strconv.Itoa(info.Staged))
		res.WriteString(ICON_VCS_STAGED)
		res.WriteString(" ")
	}

	if info.Unstaged > 0 {
		res.WriteString(strconv.Itoa(info.Unstaged))
		res.WriteString(ICON_VCS_UNSTAGED)
		res.WriteString(" ")
	}

	if info.Untracked > 0 {
		res.WriteString(strconv.Itoa(info.Untracked))
		res.WriteString(ICON_VCS_UNTRACKED)
		res.WriteString(" ")
	}

	return res.String()
}
