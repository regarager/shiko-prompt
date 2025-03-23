#!/usr/bin/env python3

import os
import json
from generate import generate
from os.path import isfile
from util import prompt, yn

print(
    """
__        __   _                          _
\\ \\      / /__| | ___ ___  _ __ ___   ___| |
 \\ \\ /\\ / / _ \\ |/ __/ _ \\| '_ ` _ \\ / _ \\ |
  \\ V  V /  __/ | (_| (_) | | | | | |  __/_|
   \\_/\\_/ \\___|_|\\___\\___/|_| |_| |_|\\___(_)

"""
)

root = os.path.expanduser("~") + "/shiko-prompt/"


def pick_prebuilt():
    themes = root + "themes/"
    prebuilt_themes = themes + "prebuilt/"

    previous = []
    prebuilt = []
    for filename in os.listdir(themes):
        file_path = os.path.join(themes, filename)

        if isfile(file_path):
            previous.append(file_path)

    for filename in os.listdir(prebuilt_themes):
        file_path = os.path.join(prebuilt_themes, filename)

        if isfile(file_path):
            prebuilt.append(file_path)

    print(f"Previous themes: {len(previous)}")
    for i in range(len(previous)):
        print(f"{i + 1}. {previous[i]}")
    print(f"Prebuilt themes: {len(prebuilt)}")
    for i in range(len(prebuilt)):
        print(f"{i + 1}. {prebuilt[i]}")

    print("Would you like to use a previously generated theme or a prebuilt theme?")
    prev_or_prebuilt = prompt(["Previously generated", "Prebuilt"])

    theme = ""

    print("Pick a theme:")
    if prev_or_prebuilt == 1:
        theme = previous[prompt(previous) - 1]
    elif prev_or_prebuilt == 2:
        theme = prebuilt[prompt(prebuilt) - 1]

    try:
        with open(theme) as f:
            json_cfg = dict(json.load(f))

            cfg = generate(json_cfg)

            print("Config generated!")
            print(cfg)
            save = yn("Would you like to save this prompt?")

            if save:
                with open(root + "prompt.zsh", "w") as f:
                    f.write(cfg)
                    return
            else:
                print("Changes discarded. Exiting...")
                exit(0)
    except Exception as e:
        print(f"An error occurred: {e}")

    pass


def build_own():
    pass


if __name__ == "__main__":
    options_funcs = [pick_prebuilt, build_own, exit]
    options = ["Use a prebuilt theme", "Build your own theme", "Exit"]

    choice = prompt(options)

    options_funcs[int(choice) - 1]()
    print("Run 'source ~/.zshrc' to see the new changes!")
