package main

import (
	"fmt"
	"os"
	"strings"
)

// highlight only the last part of the cwd
// nothing is highlighted if the cwd is / or ~
func highlight_last(cwd string) string {
	index := strings.LastIndex(cwd, "/")

	if index < 0 {
		return cwd
	}

	return fmt.Sprintf("%s%s", cwd[:index], bold(cwd[index:]))
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
		text(COLOR1),
		cwd,
	)
}
