# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "numpy",
# ]
# ///
import time
from textwrap import dedent

import numpy as np

MAX_LEN = 98


def char_to_idx(s: str) -> int:
    if s.isupper():
        return ord(s) - ord("A") + 26
    else:
        assert s.islower()
        return ord(s) - ord("a")


def main() -> None:
    names, rules = (
        dedent(
            """
    Khara,Xaryt,Noxer,Kharax

    r > v,e,a,g,y
    a > e,v,x,r,g
    e > r,x,v,t
    h > a,e,v
    g > r,y
    y > p,t
    i > v,r
    K > h
    v > e
    B > r
    t > h
    N > e
    p > h
    H > e
    l > t
    z > e
    X > a
    n > v
    x > z
    T > i
    """
        )
        .strip()
        .split("\n\n")
    )

    matrix = np.zeros((2 * 26, 2 * 26), dtype=object)
    r = {}
    for line in filter(None, rules.splitlines()):
        lhs, rhs = line.split(" > ")
        row = char_to_idx(lhs)
        for c in rhs.split(","):
            r.setdefault(lhs, set()).add(c)
            col = char_to_idx(c)
            matrix[col, row] += 1

    prefixes = []
    for n in sorted(names.split(","), key=len):
        if all(c2 in r[c1] for c1, c2 in zip(n, n[1:])) and not any(
            n.startswith(p) for p in prefixes
        ):
            prefixes.append(n)

    start = time.perf_counter()
    total = 0
    for p in prefixes:
        v = np.zeros(2 * 26, dtype=object)
        v[char_to_idx(p[-1])] += 1

        for l in range(len(p), MAX_LEN + 1):
            if l >= 7:
                total += int(sum(v))
            v = matrix @ v
    print(total, f"({total.bit_length()} bits)")
    print("Runtime:", 1 * (time.perf_counter() - start), "s")


if __name__ == "__main__":
    main()
