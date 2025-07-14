package main

import (
	"fmt"
	"os"
	"strings"
)

func darken_prefix(prefix string) string {
	color := darken(COLOR1, CWD_DARKEN_FACTOR)

	return fmt.Sprintf("%s%s%s", fg(color), prefix, RESET)
}

// highlight only the last part of the cwd
// nothing is highlighted if the cwd is / or ~
func highlight_last(cwd string) string {
	index := max(0, strings.LastIndex(cwd, "/"))

	prefix := cwd[:index]

	if CWD_DARKEN {
		return fmt.Sprintf("%s%s%s", darken_prefix(prefix), fg(COLOR1), bold(cwd[index:]))
	}

	return fmt.Sprintf("%s%s", prefix, bold(cwd[index:]))
}

// TODO: add option to not truncate $HOME
func cwd_info() string {
	output, err := os.Getwd()

	cwd := string(output)

	if err != nil {
		return ""
	}

	output, err = os.UserHomeDir()

	home_dir := string(output)

	if err != nil {
		return cwd
	}

	if strings.HasPrefix(cwd, home_dir) {
		return "~" + strings.TrimPrefix(cwd, home_dir)
	}

	return cwd
}

func section_cwd() string {
	cwd := cwd_info()

	if CWD_HIGHLIGHT_LAST {
		cwd = highlight_last(cwd)
	} else {
		cwd = bold(cwd)
	}

	return fmt.Sprintf(
		"%s%s ",
		fg(COLOR1),
		cwd,
	)
}
