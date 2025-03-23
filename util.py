from typing import List


def prompt(options: List[str]) -> int:
    while True:
        for i in range(len(options)):
            print(f"{i + 1}. {options[i]}")
        try:
            choice = int(input("> "))
            if choice < 1 or choice > len(options):
                raise Exception()
            return choice
        except KeyboardInterrupt:
            print("Exiting...")
            exit()
        except:
            print("Invalid choice, please try again")


def yn(prompt: str, default_value=True) -> bool:
    while True:
        choice = input(f"{prompt} ({"Y/n" if default_value else "y/N"}) ")

        if choice.strip() == "":
            return default_value
        elif choice.strip().lower() == "y":
            return True
        elif choice.strip().lower() == "n":
            return False
        print("Invalid choice, please try again")
