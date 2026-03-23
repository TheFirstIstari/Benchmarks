import time
import numpy as np

ITERATIONS = 100
VECTOR_SIZE = 1024


def main():
    print("Python ML Benchmark")

    a = np.linspace(0, 1, VECTOR_SIZE, dtype=np.float32)
    b = np.linspace(1, 0, VECTOR_SIZE, dtype=np.float32)
    c = np.empty(VECTOR_SIZE, dtype=np.float32)

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        c = a * b
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000
    print(f"Element-wise mul: {total_ms:.2f} ms")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        dot = np.dot(a, b)
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000
    print(f"Dot product: {total_ms:.2f} ms (result: {dot:.4f})")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        c = a * 0.9 + b * 0.1
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000
    print(f"Lerp (mix): {total_ms:.2f} ms")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        c = 1.0 / (1.0 + a)
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000
    print(f"Sigmoid: {total_ms:.2f} ms")


if __name__ == "__main__":
    main()
