#!/usr/bin/env python3
import time
import numpy as np

N = 2000
ITERATIONS = 5


def main():
    print(f"Python Matrix Benchmark ({N}x{N}, {ITERATIONS} iterations)")

    rng = np.random.RandomState(42)
    A = rng.random((N, N))
    B = rng.random((N, N))
    checksum = 0.0

    ops = N * N * N * 2

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        C = A @ B
    t1 = time.perf_counter()

    ms = (t1 - t0) / ITERATIONS * 1000
    gflops = (ops * ITERATIONS) / (t1 - t0) / 1e9
    print(f"Multiply: {ms:.2f} ms ({gflops:.2f} GFLOPS)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        T = A.T
        checksum += T.sum()
    t1 = time.perf_counter()
    ms = (t1 - t0) / ITERATIONS * 1000
    print(f"Transpose: {ms:.2f} ms")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        C = A + B
    t1 = time.perf_counter()
    ms = (t1 - t0) / ITERATIONS * 1000
    print(f"Add: {ms:.2f} ms")


if __name__ == "__main__":
    main()
