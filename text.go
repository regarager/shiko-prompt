package main

import "fmt"

func text(color string) string {
	return fmt.Sprintf("%%F{%s}", color)
}

func bg(color string) string {
	return fmt.Sprintf("%%K{%s}", color)
}

func bold(text string) string {
	return fmt.Sprintf("%%B%s%%b", text)
}
