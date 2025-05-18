package main

import (
	"fmt"
)

func text(color string) string {
	return fmt.Sprintf("%%F{%s}", color)
}

func bg(color string) string {
	return fmt.Sprintf("%%K{%s}", color)
}

func main() {
	fmt.Printf("%s%s%s%s%s ", section_cwd(), section_git(), text(COLOR3), ICON_ARROW, RESET)
}
