#!/usr/bin/env python3
import math
import time

ITERATIONS = 10000000


def main():
    print(f"Python Math Benchmark ({ITERATIONS} iterations)")

    t0 = time.perf_counter()
    result = 0.0
    for i in range(1, ITERATIONS):
        result += math.sqrt(i) + math.sin(i) + math.cos(i)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Trig functions: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for i in range(1, ITERATIONS):
        result += math.log(i) + math.exp(i % 10)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Exp/log functions: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for i in range(1, ITERATIONS):
        x = i
        result += x * x + x * x * x + math.sqrt(x * x + 1.0)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Arithmetic: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    _ = result


if __name__ == "__main__":
    main()
