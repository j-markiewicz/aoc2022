score_a = {
    "A X": 1 + 3,
    "A Y": 2 + 6,
    "A Z": 3 + 0,
    "B X": 1 + 0,
    "B Y": 2 + 3,
    "B Z": 3 + 6,
    "C X": 1 + 6,
    "C Y": 2 + 0,
    "C Z": 3 + 3,
}

score_b = {
    "A X": 3 + 0,
    "A Y": 1 + 3,
    "A Z": 2 + 6,
    "B X": 1 + 0,
    "B Y": 2 + 3,
    "B Z": 3 + 6,
    "C X": 2 + 0,
    "C Y": 3 + 3,
    "C Z": 1 + 6,
}

with open("./input.txt") as f:
    games = f.readlines()
    total_a = 0
    total_b = 0

    for game in games:
        total_a += score_a[game.strip()]
        total_b += score_b[game.strip()]

    print(f"1: {total_a}")
    print(f"2: {total_b}")
