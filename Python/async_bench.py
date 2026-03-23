#!/usr/bin/env python3
import time
import concurrent.futures
import threading
from multiprocessing import Pool

NUM_THREADS = 8
OPS_PER_THREAD = 10_000_000

counter = 0
counter_lock = threading.Lock()


def atomic_increment():
    global counter
    for _ in range(OPS_PER_THREAD):
        with counter_lock:
            counter += 1


def parallel_sum(start, end):
    total = 0
    for i in range(start, end):
        total += i * i
    return total


def thread_pool_task(args):
    start, end = args
    total = 0
    for i in range(start, end):
        total += i * i
    return total


def main():
    print(
        f"Python Concurrency Benchmark ({NUM_THREADS} threads, {OPS_PER_THREAD} ops/thread)"
    )

    global counter
    counter = 0
    t0 = time.perf_counter()
    with concurrent.futures.ThreadPoolExecutor(max_workers=NUM_THREADS) as executor:
        futures = [executor.submit(atomic_increment) for _ in range(NUM_THREADS)]
        for f in futures:
            f.result()
    t1 = time.perf_counter()
    total_ops = NUM_THREADS * OPS_PER_THREAD
    ms = (t1 - t0) * 1000
    print(f"Atomic: {ms:.2f} ms ({total_ops / (t1 - t0):.0f} ops/sec)")

    t0 = time.perf_counter()
    with concurrent.futures.ThreadPoolExecutor(max_workers=NUM_THREADS) as executor:
        futures = [
            executor.submit(parallel_sum, i * OPS_PER_THREAD, (i + 1) * OPS_PER_THREAD)
            for i in range(NUM_THREADS)
        ]
        sum(f.result() for f in futures)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Parallel sum: {ms:.2f} ms ({total_ops / (t1 - t0):.0f} ops/sec)")

    t0 = time.perf_counter()
    with concurrent.futures.ThreadPoolExecutor(max_workers=NUM_THREADS) as executor:
        futures = [
            executor.submit(
                thread_pool_task, (i * OPS_PER_THREAD, (i + 1) * OPS_PER_THREAD)
            )
            for i in range(NUM_THREADS)
        ]
        sum(f.result() for f in futures)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Thread pool: {ms:.2f} ms ({total_ops / (t1 - t0):.0f} ops/sec)")


if __name__ == "__main__":
    main()
