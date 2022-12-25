SNAFU = {
    "=": -2,
    "-": -1,
    "0": 0,
    "1": 1,
    "2": 2
}

UFANS = {
    -2: "=",
    -1: "-",
    0: "0",
    1: "1",
    2: "2",
}

with open("input.txt") as f:
    s = 0
    for line in f.readlines():
        i = 1
        n = 0
        for c in line.strip()[::-1]:
            n += i * SNAFU[c]
            i *= 5
        s += n

    os = s

    n = []
    i = 5 ** 25
    for _ in range(25):
        d = s // i
        s -= i * d
        i //= 5
        n.append(d)

    for i in range(len(n) - 1, -1, -1):
        if n[i] > 2:
            n[i] -= 5
            n[i - 1] += 1

    sn = "".join(map(lambda d: UFANS[d], n))

    sn += "0"
    while sn[0] == "0":
        sn = sn[1:]

    di = 1
    dn = 0
    for dc in sn.strip()[::-1]:
        dn += di * SNAFU[dc]
        di *= 5

    assert dn == os, "encoding failed"

    print(f"1: {sn}")

    print(f"2: https://github.com/j-markiewicz/aoc2022")
