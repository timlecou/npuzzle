import sys
import random
import os

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

def generate_invalid_puzzle(size: int):
    max_number = size * size
    li = list(range(0, max_number))
    random.shuffle(li)
    try:
        i = 0
        while 1:
            path = f"puzzles/invalid-puz-{size}-{i}.txt"
            if os.path.exists(path):
                i += 1
            else:
                with open(path, 'x') as f:
                    f.write(f"{size}")
                    idx = 0
                    tmp_size = size - 1
                    while idx < max_number:
                        f.write(f"\n")
                        f.write(f" ".join(map(str, li[idx:tmp_size])))
                        idx = tmp_size
                        tmp_size += size
                    print(bcolors.OKGREEN + f"INVALID: {path} generated !")
                    exit(0)
    except FileNotFoundError:
        print(bcolors.FAIL + "The 'puzzles' directory does not exist")

def generate_valid_puzzle(size: int) -> None:
    max_number = size * size
    li = list(range(0, max_number))
    random.shuffle(li)
    try:
        i = 0
        while 1:
            path = f"puzzles/valid-puz-{size}-{i}.txt"
            if os.path.exists(path):
                i += 1
            else:
                with open(path, 'x') as f:
                    f.write(f"{size}")
                    idx = 0
                    tmp_size = size
                    while idx < max_number:
                        f.write(f"\n")
                        f.write(f" ".join(map(str, li[idx:tmp_size])))
                        idx = tmp_size
                        tmp_size += size
                    print(bcolors.OKGREEN + f"VALID: {path} generated !")
                    exit(0)
    except FileNotFoundError:
        print(bcolors.FAIL + "The 'puzzles' directory does not exist")

def main():
    if len(sys.argv) != 3:
        print(bcolors.FAIL + "You must provide 2 arguments: <type> <size>")
    if sys.argv[1] == "-v":
        generate_valid_puzzle(int(sys.argv[2]))
    elif sys.argv[1] == "-i":
        generate_invalid_puzzle(int(sys.argv[2]))
    elif sys.argv[1] == "-h":
        print(f"\n\nCommands: -i or -v <size>\n")
        print(f"-i: generate invalid puzzle")
        print(f"-v: generate valid puzzle")
        print(f"size: size of puzzle sides\n\n")
    else:
        print(bcolors.FAIL + f"'{sys.argv[1][1:]}': Invalid command")

main()