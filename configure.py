import subprocess
import re
import math
from random import randint


def yn(prompt: str, default: bool = True):
    res = "asdf"

    while len(res) != 0 or res not in "yn":
        res = input(prompt + " " + ("Y/n" if default else "y/N") + " ").strip().lower()

    if len(res) == 0:
        return default

    return res == "y"


def is_hexcode(s: str):
    return re.match("^#[0-9a-fA-F]{6}$", s) != None


def string(prompt: str, default: str | None):
    res = "asdf"

    while len(res) != 0 or not is_hexcode(res):
        display = f" (default: {default})" if default != None else ""
        res = input(f"{prompt}{display}: ").strip().lower()

        if len(res) == 0:
            return default

    return res


def number(
    prompt: str,
    default: float | None,
    lower: float = -math.inf,
    upper: float = math.inf,
):
    segments = []

    if default != None:
        segments.append(f"default: {default}")
    if lower > -math.inf:
        segments.append(f"min: {lower}")
    if upper < math.inf:
        segments.append(f"max: {upper}")

    display = "(" + ", ".join(segments) + ")" if len(segments) > 0 else ""
    while True:
        try:
            res = input(f"{prompt}{display}: ").strip()

            if res == "":
                if default != None:
                    return default
                continue

            x = float(res)

            if x > lower and x < upper:
                return x
        except:
            pass


def main():
    conf = {}
    conf["COLOR1"] = string("Enter CWD color", "#2bd4ff")
    conf["COLOR2"] = string("Enter VCS branch color", "#00e600")
    conf["COLOR3"] = string("Enter arrow color", "#b5fd0d")
    conf["COLOR_VCS_CHANGE"] = string("Enter VCS changes color", "#f4d03f")
    # conf["CWD_HIGHLIGHT_LAST"] = yn("Highlight the last section of CWD?")
    # conf["CWD_DARKEN"] = yn("Darken the prefix of the CWD (before last)?")
    # conf["CWD_DARKEN_FACTOR"] = (
    #     number("Enter darkening factor", default=0.25, lower=0, upper=1)
    #     if conf["CWD_DARKEN"]
    #     else 0
    # )

    # TODO: figure out how to get other options working
    # currently cant configure non string
    print("Icons coming soon")

    rand = randint(10**5, 10**6 - 1)

    filename = f"shiko-prompt-{rand}"

    flags = " ".join(map(lambda x: f"-X main.{x}={conf[x]}", conf.keys()))
    args = ["go", "build", f'-ldflags="{flags}"', f"-o={filename}", "*.go"]

    print("Running command ", end="")
    print(" ".join(args))

    subprocess.run(" ".join(args), shell=True)

    print(f"Compiled binary to {filename}")
    print(
        f"Test it out by opening a new shell with 'zsh -idf' and running 'PS1=$(./{filename})'"
    )
    pass


if __name__ == "__main__":
    main()
