import re
from copy import deepcopy

with open("input.txt") as f:
    input = f.read()

    num_stacks = int(input.split("\n\n")[0].strip().split(" ")[-1], 10)

    crates = [[] for _ in range(num_stacks)]
    for line in input.split("\n\n")[0].splitlines()[:-1]:
        for i in range(num_stacks):
            crate = line[1 + i * 4]
            crates[i].append(crate if crate != " " else None)
            i += 1

    for crate in crates:
        while crate[0] is None:
            crate.pop(0)

    crates_1 = deepcopy(crates)
    crates_2 = deepcopy(crates)

    for line in input.split("\n\n")[1].splitlines():
        match = re.match(r"move (\d+) from (\d+) to (\d+)", line)
        i = int(match.group(1), 10)
        f = int(match.group(2), 10)
        t = int(match.group(3), 10)

        for _ in range(i):
            crate = crates_1[f - 1].pop(0)
            crates_1[t - 1].insert(0, crate)

        move = crates_2[f - 1][0:i][::-1]
        for crate in move:
            crates_2[t - 1].insert(0, crate)
        for _ in range(i):
            crates_2[f - 1].pop(0)

    print("1: ", end="")
    for crate in crates_1:
        print(crate[0], end="")
    print()

    print("2: ", end="")
    for crate in crates_2:
        print(crate[0], end="")
    print()
