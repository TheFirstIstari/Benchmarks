#!/usr/bin/env python3
import time
import os

ITERATIONS = 1000
FILE_SIZE = 5000000


def main():
    print(f"Python File I/O Benchmark ({ITERATIONS} iterations)")

    data = bytes([ord("a") + (i % 26) for i in range(FILE_SIZE)])

    if os.path.exists("/tmp/bench_io.dat"):
        os.remove("/tmp/bench_io.dat")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        with open("/tmp/bench_io.dat", "wb") as f:
            f.write(data)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Write: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        with open("/tmp/bench_io.dat", "rb") as f:
            _ = f.read()
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Read: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        with open("/tmp/bench_io.dat", "r") as f:
            for line in f:
                pass
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Read lines: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        with open("/tmp/bench_io.dat", "rb") as f:
            f.seek(FILE_SIZE // 2)
            _ = f.read(1024)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Random access: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    if os.path.exists("/tmp/bench_io.dat"):
        os.remove("/tmp/bench_io.dat")


if __name__ == "__main__":
    main()
