package main

import (
	"fmt"
	"os"
	"strings"
)

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
	return fmt.Sprintf(
		"%s%s ",
		text(COLOR1),
		fmt.Sprintf("%%B%s%%b", cwd),
	)
}
