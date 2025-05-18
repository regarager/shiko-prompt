package main

import (
	"fmt"
)

func main() {
	fmt.Printf("%s%s%s%s%s ", section_cwd(), section_git(), text(COLOR3), ICON_ARROW, RESET)
}
