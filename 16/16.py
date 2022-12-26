import re
from collections import namedtuple
from itertools import permutations

cave = {}
start = "AA"

Valve = namedtuple("Valve", "name rate targets")

with open("./input.txt") as f:
    for line in f.readlines():
        matches = re.match(
            r"Valve (?P<valve>[A-Z]{2}) has flow rate=(?P<rate>\d+); " +
            r"tunnels? leads? to valves? (?P<targets>([A-Z]{2}(, )?)+)",
            line
        )

        valve = matches.group("valve")
        rate = int(matches.group("rate"), 10)
        targets = matches.group("targets").split(", ")

        cave[valve] = Valve(valve, rate, targets)

dist = {(v.name, v.name): 1 for v in cave.values()}

pairs = []
for v1 in cave.keys():
    for v2 in cave.keys():
        pairs.append((v1, v2))
        if (v1, v2) not in dist:
            if v2 in cave[v1].targets:
                dist[(v1, v2)] = 2

for v1 in cave.keys():
    for v2 in cave.keys():
        for v3 in cave.keys():
            if (v1, v3) not in dist:
                if v2 in cave[v1].targets and v3 in cave[v2].targets:
                    dist[(v1, v3)] = 3

for v1 in cave.keys():
    for v2 in cave.keys():
        for v3 in cave.keys():
            for v4 in cave.keys():
                if (v1, v4) not in dist:
                    if v2 in cave[v1].targets and v3 in cave[v2].targets and v4 in cave[v3].targets:
                        dist[(v1, v4)] = 4

for v1 in cave.keys():
    for v2 in cave.keys():
        for v3 in cave.keys():
            for v4 in cave.keys():
                for v5 in cave.keys():
                    if (v1, v5) not in dist:
                        if v2 in cave[v1].targets and v3 in cave[v2].targets and v4 in cave[v3].targets and v5 in cave[v4].targets:
                            dist[(v1, v5)] = 5

for v1 in cave.keys():
    for v2 in cave.keys():
        for v3 in cave.keys():
            for v4 in cave.keys():
                for v5 in cave.keys():
                    for v6 in cave.keys():
                        if (v1, v6) not in dist:
                            if v2 in cave[v1].targets and v3 in cave[v2].targets and v4 in cave[v3].targets and v5 in cave[v4].targets and v6 in cave[v5].targets:
                                dist[(v1, v6)] = 6

for v1 in cave.keys():
    for v2 in cave.keys():
        for v3 in cave.keys():
            for v4 in cave.keys():
                for v5 in cave.keys():
                    for v6 in cave.keys():
                        for v7 in cave.keys():
                            if (v1, v7) not in dist:
                                if v2 in cave[v1].targets and v3 in cave[v2].targets and v4 in cave[v3].targets and v5 in cave[v4].targets and v6 in cave[v5].targets and v7 in cave[v6].targets:
                                    dist[(v1, v7)] = 7

for v1 in cave.keys():
    for v2 in cave.keys():
        for v3 in cave.keys():
            for v4 in cave.keys():
                for v5 in cave.keys():
                    for v6 in cave.keys():
                        for v7 in cave.keys():
                            for v8 in cave.keys():
                                if (v1, v8) not in dist:
                                    if v2 in cave[v1].targets and v3 in cave[v2].targets and v4 in cave[v3].targets and v5 in cave[v4].targets and v6 in cave[v5].targets and v7 in cave[v6].targets and v8 in cave[v7].targets:
                                        dist[(v1, v8)] = 8

# for v1 in cave.keys():
#     for v2 in cave.keys():
#         for v3 in cave.keys():
#             for v4 in cave.keys():
#                 for v5 in cave.keys():
#                     for v6 in cave.keys():
#                         for v7 in cave.keys():
#                             for v8 in cave.keys():
#                                 for v9 in cave.keys():
#                                     if (v1, v9) not in dist:
#                                         if v2 in cave[v1].targets and v3 in cave[v2].targets and v4 in cave[v3].targets and v5 in cave[v4].targets and v6 in cave[v5].targets and v7 in cave[v6].targets and v8 in cave[v7].targets and v9 in cave[v8].targets:
#                                             dist[(v1, v9)] = 9

# for v1 in cave.keys():
#     for v2 in cave.keys():
#         for v3 in cave.keys():
#             for v4 in cave.keys():
#                 for v5 in cave.keys():
#                     for v6 in cave.keys():
#                         for v7 in cave.keys():
#                             for v8 in cave.keys():
#                                 for v9 in cave.keys():
#                                     for v10 in cave.keys():
#                                         if (v1, v10) not in dist:
#                                             if v2 in cave[v1].targets and v3 in cave[v2].targets and v4 in cave[v3].targets and v5 in cave[v4].targets and v6 in cave[v5].targets and v7 in cave[v6].targets and v8 in cave[v7].targets and v9 in cave[v8].targets and v10 in cave[v9].targets:
#                                                 dist[(v1, v10)] = 10

assert len(dist.keys()) == len(pairs), "add more for loops"

route = []
v_working = [v.name for v in cave.values() if v.rate > 0]

max_tpl = 0
max_route = []

for route in permutations(v_working):
    ppm = 0
    tpl = 0
    i = 0
    skip = 0
    last = "AA"

    for _ in range(30):
        tpl += ppm

        if skip > 0:
            skip -= 1
            continue

        if i < len(route):
            ppm += cave[route[i]].rate
            skip = dist[(last, route[i])]
            i += 1

    if route == ("DD", "BB", "JJ", "HH", "EE", "CC"):
        print(tpl, ppm, dist[("DD", "BB")])

    if tpl > max_tpl:
        max_tpl = tpl
        max_route = route

print(f"1: {max_tpl} ({' -> '.join(max_route)})")
