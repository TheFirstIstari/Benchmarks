#!/usr/bin/env python3
import time
import random

N = 5_000_000
ITERATIONS = 5


def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)


def heapsort(arr):
    import heapq

    heapq.heapify(arr)
    return [heapq.heappop(arr) for _ in range(len(arr))]


def mergesort(arr):
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left = mergesort(arr[:mid])
    right = mergesort(arr[mid:])
    result = []
    i = j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1
    result.extend(left[i:])
    result.extend(right[j:])
    return result


def main():
    print(f"Python Sort Benchmark (N={N}, {ITERATIONS} iterations)")

    original = [random.randint(0, 1_000_000) for _ in range(N)]

    t0 = time.perf_counter()
    arr = original.copy()
    arr.sort()
    t1 = time.perf_counter()
    print(
        f"Sort (stdlib): {(t1 - t0) * 1000:.2f} ms ({N / (t1 - t0):.0f} elements/sec)"
    )

    t0 = time.perf_counter()
    arr = quicksort(original)
    t1 = time.perf_counter()
    print(f"Quicksort: {(t1 - t0) * 1000:.2f} ms ({N / (t1 - t0):.0f} elements/sec)")

    t0 = time.perf_counter()
    arr = heapsort(original.copy())
    t1 = time.perf_counter()
    print(f"Heapsort: {(t1 - t0) * 1000:.2f} ms ({N / (t1 - t0):.0f} elements/sec)")

    t0 = time.perf_counter()
    arr = mergesort(original)
    t1 = time.perf_counter()
    print(f"Mergesort: {(t1 - t0) * 1000:.2f} ms ({N / (t1 - t0):.0f} elements/sec)")


if __name__ == "__main__":
    main()
