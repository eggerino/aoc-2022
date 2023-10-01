import sys


with open(sys.argv[1], "r") as f:
    lines = [line.strip() for line in f]


def is_unique(s: str):
    return len(set(s)) == len(s)


def find_marker(s: str, n: int):
    for i in range(len(s) - n):
        if is_unique(s[i:i+n]):
            return i + n
    return None


print(f"Part 1: {find_marker(lines[0], 4)}")
print(f"Part 2: {find_marker(lines[0], 14)}")
