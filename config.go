package main

// needs to be var for compile args
var (
	// options
	CWD_HIGHLIGHT_LAST = true
	CWD_DARKEN         = true
	CWD_DARKEN_FACTOR  = 0.25

	// colors
	COLOR1           = "#2bd4ff"
	COLOR2           = "#00e600"
	COLOR3           = "#b5fd0d"
	COLOR_VCS_CHANGE = "#f4d03f"

	// terminal characters
	RESET = "%f%k"

	// icons
	ICON_LEFT          = ""
	ICON_RIGHT         = ""
	ICON_ARROW         = "➔"
	ICON_VCS_BRANCH    = ""
	ICON_VCS_AHEAD     = ""
	ICON_VCS_BEHIND    = ""
	ICON_VCS_STAGED    = "+"
	ICON_VCS_UNSTAGED  = "*"
	ICON_VCS_UNTRACKED = "?"
)
