import sys


with open(sys.argv[1], "r") as f:
    lines = [line.strip() for line in f]


def build_dir_map(lines: list[str]):
    dir_stack = []
    dir_map = {}

    for line in lines:
        match line.split():
            case ["$", "cd", dir]:
                match dir:
                    case "/":
                        dir_stack = ["/"]
                    case "..":
                        dir_stack.pop()
                    case _:
                        dir_stack.append(dir)
            case [size, _] if size.isnumeric():
                path = ""
                for dir in dir_stack:
                    path = "".join((path, dir))
                    dir_map[path] = dir_map.get(path, 0) + int(size)

    return dir_map


dir_map = build_dir_map(lines)
print(f"Part 1: {sum((x for x in dir_map.values() if x <= 100_000))}")
currently_free_space = 70_000_000 - dir_map["/"]
need_to_free = 30_000_000 - currently_free_space
print(f"Part 2: {min((x for x in dir_map.values() if x >= need_to_free))}")
