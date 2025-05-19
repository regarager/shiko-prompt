package main

import (
	"fmt"
	"strconv"
	"strings"
)

const RESET = "%f%k"

func fg(color string) string {
	return fmt.Sprintf("%%F{%s}", color)
}

func bg(color string) string {
	return fmt.Sprintf("%%K{%s}", color)
}

func bold(text string) string {
	return fmt.Sprintf("%%B%s%%b", text)
}

func darken(hex string, factor float64) string {
	hex = strings.TrimPrefix(hex, "#")

	r, _ := strconv.ParseUint(hex[0:2], 16, 8)
	g, _ := strconv.ParseUint(hex[2:4], 16, 8)
	b, _ := strconv.ParseUint(hex[4:6], 16, 8)

	factor = max(1-factor, 0)

	r = max(min(uint64(float64(r)*factor), 0xff), 0)
	g = max(min(uint64(float64(g)*factor), 0xff), 0)
	b = max(min(uint64(float64(b)*factor), 0xff), 0)

	return fmt.Sprintf("#%02X%02X%02X", r, g, b)
}
