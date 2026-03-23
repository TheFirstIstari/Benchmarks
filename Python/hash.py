#!/usr/bin/env python3
import time

ITERATIONS = 10000
STR_LEN = 1000


def hash_sdbm(s):
    hash_val = 0
    for c in s:
        hash_val = ord(c) + (hash_val << 6) + (hash_val << 16) - hash_val
    return hash_val


def hash_djb2(s):
    hash_val = 5381
    for c in s:
        hash_val = ((hash_val << 5) + hash_val) + ord(c)
    return hash_val


def hash_fnv(s):
    hash_val = 0x811C9DC5
    for c in s:
        hash_val ^= ord(c)
        hash_val = (hash_val * 0x01000193) & 0xFFFFFFFF
    return hash_val


def main():
    print(f"Python Hashing Benchmark ({ITERATIONS} iterations)")

    data = "a" * STR_LEN
    result = 0

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        result += hash_sdbm(data)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"SDBM: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        result += hash_djb2(data)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"DJB2: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        result += hash_fnv(data)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"FNV: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    if result == 0:
        print("")


if __name__ == "__main__":
    main()
