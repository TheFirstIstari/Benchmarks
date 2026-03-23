#!/usr/bin/env python3
import time

STR_LEN = 1000
ITERATIONS = 100000


def hash_djb2(s):
    h = 5381
    for c in s:
        h = ((h << 5) + h) + ord(c)
    return h & 0xFFFFFFFF


def count_splits(s, delim):
    return s.count(delim) + 1


def main():
    print(f"Python String Operations ({ITERATIONS} iterations)")

    test_str = "".join(chr(ord("a") + (i % 26)) for i in range(STR_LEN))

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        hash_djb2(test_str)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Hash: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        test_str[::-1]
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Reverse: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        count_splits(test_str, ",")
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Split: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")


if __name__ == "__main__":
    main()
