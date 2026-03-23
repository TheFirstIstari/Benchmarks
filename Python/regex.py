#!/usr/bin/env python3
import time
import re

ITERATIONS = 10
STR_LEN = 100000


def main():
    print(f"Python Regex Benchmark ({ITERATIONS} iterations)")

    test_str = "".join(chr(ord("a") + (i % 26)) for i in range(STR_LEN))

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        re.search(r"xyz", test_str)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Find: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        re.findall(r"[aeiou]", test_str)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Count: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        re.search(r"[a-z]+@[a-z]+\.[a-z]+", test_str[:100])
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Email match: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        re.search(r"[0-9]{3}-[0-9]{3}-[0-9]{4}", test_str)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Complex pattern: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")


if __name__ == "__main__":
    main()
