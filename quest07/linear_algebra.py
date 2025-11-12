# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "numpy",
# ]
# ///
import time
from collections import Counter

import numpy as np

MAX_LEN = 11


def char_to_idx(s: str) -> int:
    if s.isupper():
        return ord(s) - ord("A") + 26
    else:
        assert s.islower()
        return ord(s) - ord("a")


def main() -> None:
    names, rules = open("src/part3.txt").read().split("\n\n")

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

    total = 0
    transformations = {}

    for l0 in range(1, MAX_LEN + 1):
        transformation = np.zeros((2 * 26, 2 * 26), dtype=object)
        current = np.identity(2 * 26, dtype=object)

        for l in range(l0, MAX_LEN + 1):
            if l >= 7:
                transformation += current
            current = current @ matrix

        sum_row_vec = np.ones((1, 2 * 26), dtype=object)
        result = sum_row_vec @ transformation
        transformations[l0] = result.flatten()

    print(Counter(map(len, prefixes)))

    start = time.perf_counter()
    for p in prefixes:
        total += transformations[len(p)][char_to_idx(p[-1])]

    print("Total:", total)
    print("Runtime:", format(1e6 * (time.perf_counter() - start), ".2f"), "us")


if __name__ == "__main__":
    main()
