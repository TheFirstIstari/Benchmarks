#!/usr/bin/env python3
"""CPU benchmark: integer throughput, branch prediction, memory latency, SIMD, atomics."""
import time, random

N = 10_000_000          # integer ops
BRANCH_N = 100_000_000  # branch prediction
POINTER_N = 50_000_000  # memory latency
SIMD_N = 50_000_000     # vector ops

random.seed(42)

def fmt_ms(dt):
    return dt * 1000

# 1. Integer arithmetic
def bench_fibonacci():
    def fib(n):
        a, b = 0, 1
        for _ in range(n):
            a, b = b, a + b
        return a

    t0 = time.perf_counter()
    result = fib(35)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"cpu_fibonacci: {ms:.2f} ms")
    _ = result

def bench_prime_sieve():
    limit = 10_000_000
    t0 = time.perf_counter()
    sieve = bytearray(b'\x01') * (limit + 1)
    sieve[0:2] = b'\x00\x00'
    count = 0
    for i in range(2, limit + 1):
        if sieve[i]:
            count += 1
            if i * i <= limit:
                step = i
                start = i * i
                sieve[start:limit+1:step] = b'\x00' * ((limit - start) // step + 1)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"cpu_prime_sieve: {ms:.2f} ms ({count} primes)")

def bench_popcount():
    data = [random.randint(0, 2**31-1) for _ in range(N)]
    t0 = time.perf_counter()
    total = sum(bin(x).count('1') for x in data)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"cpu_popcount: {ms:.2f} ms ({len(data) / (ms / 1000):.0f} ops/sec)")
    _ = total

# 2. Branch prediction
def bench_branch_prediction():
    sorted_data = list(range(BRANCH_N))
    unsorted_data = [random.randint(0, 99) for _ in range(BRANCH_N)]

    t0 = time.perf_counter()
    total_sorted = sum(x for x in sorted_data if x > 50_000_000)
    t1 = time.perf_counter()
    ms_srt = (t1 - t0) * 1000

    t0 = time.perf_counter()
    total_unsorted = sum(x for x in unsorted_data if x > 50)
    t1 = time.perf_counter()
    ms_uns = (t1 - t0) * 1000

    print(f"cpu_branch_sorted: {ms_srt:.2f} ms")
    print(f"cpu_branch_unsorted: {ms_uns:.2f} ms ({ms_uns/ms_srt:.1f}x slower)")
    _ = (total_unsorted, total_sorted)

# 3. Memory latency (pointer chasing)
def bench_memory_latency():
    indices = list(range(POINTER_N))
    for i in range(POINTER_N):
        indices[i] = (i + 1) % POINTER_N
    # Shuffle (Fisher-Yates, matching C)
    for i in range(POINTER_N - 1, 0, -1):
        j = random.randint(0, i)
        indices[i], indices[j] = indices[j], indices[i]

    pos = 0
    t0 = time.perf_counter()
    for _ in range(POINTER_N):
        pos = indices[pos]
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"cpu_memory_latency: {ms:.2f} ms ({POINTER_N / (ms / 1000):.0f} traversals/sec)")
    _ = pos

# 4. SIMD via numpy
def bench_simd():
    try:
        import numpy as np
        a = np.arange(SIMD_N, dtype=np.float32)
        b = np.arange(SIMD_N, dtype=np.float32) % 100

        t0 = time.perf_counter()
        c = (a + b) * a
        t1 = time.perf_counter()
        ms = (t1 - t0) * 1000
        total = float(c[::1000].sum())
        print(f"cpu_simd_numpy: {ms:.2f} ms ({SIMD_N / (ms / 1000):.0f} ops/sec)")
    except ImportError:
        a = [float(i) for i in range(SIMD_N)]
        b = [float(i % 100) for i in range(SIMD_N)]
        c = [0.0] * SIMD_N

        t0 = time.perf_counter()
        for i in range(SIMD_N):
            c[i] = (a[i] + b[i]) * a[i]
        t1 = time.perf_counter()
        ms = (t1 - t0) * 1000
        total = sum(c[i] for i in range(0, SIMD_N, 1000))
        print(f"cpu_simd_scalar: {ms:.2f} ms ({SIMD_N / (ms / 1000):.0f} ops/sec)")
    _ = total

# 5. Atomic via threading.Lock
def bench_atomic():
    import threading

    atomic_val = [0]
    lock = threading.Lock()

    t0 = time.perf_counter()
    for _ in range(N):
        with lock:
            atomic_val[0] += 1
    t1 = time.perf_counter()
    ms_atomic = (t1 - t0) * 1000
    print(f"cpu_atomic_add: {ms_atomic:.2f} ms ({N / (ms_atomic / 1000):.0f} ops/sec)")

    regular_val = 0
    t0 = time.perf_counter()
    for _ in range(N):
        regular_val += 1
    t1 = time.perf_counter()
    ms_reg = (t1 - t0) * 1000
    print(f"cpu_nonatomic_add: {ms_reg:.2f} ms ({N / (ms_reg / 1000):.0f} ops/sec, {ms_atomic/ms_reg:.1f}x faster)")

    if threading.current_thread() is None:
        print(atomic_val, regular_val)

if __name__ == "__main__":
    print("Python CPU Benchmark")
    print("Apple M4 — 10 cores (4P+6E), 16GB unified\n")

    print("--- Integer Arithmetic ---")
    bench_fibonacci()
    bench_prime_sieve()
    bench_popcount()

    print("\n--- Branch Prediction ---")
    bench_branch_prediction()

    print("\n--- Memory Latency ---")
    bench_memory_latency()

    print("\n--- SIMD Vectorization ---")
    bench_simd()

    print("\n--- Atomic Operations ---")
    bench_atomic()
