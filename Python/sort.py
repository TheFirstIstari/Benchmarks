#!/usr/bin/env python3
"""Sort benchmarks — optimized for Python.
Python's strength is C-implemented stdlib. list.sort() (Timsort) is the right baseline.
Pure-Python sorts are included for comparison but will be 100x+ slower.
"""
import time
import random
import array

N = 5_000_000
ITERATIONS = 5


def quicksort_inplace(arr, lo=0, hi=None):
    """In-place quicksort with median-of-three pivot + insertion cutoff."""
    if hi is None:
        hi = len(arr) - 1
    while hi - lo > 32:
        mid = lo + (hi - lo) // 2
        # Median of three
        if arr[lo] > arr[mid]:
            arr[lo], arr[mid] = arr[mid], arr[lo]
        if arr[lo] > arr[hi]:
            arr[lo], arr[hi] = arr[hi], arr[lo]
        if arr[mid] > arr[hi]:
            arr[mid], arr[hi] = arr[hi], arr[mid]
        arr[mid], arr[hi] = arr[hi], arr[mid]
        pivot = arr[hi]
        i = lo - 1
        for j in range(lo, hi):
            if arr[j] <= pivot:
                i += 1
                arr[i], arr[j] = arr[j], arr[i]
        arr[i + 1], arr[hi] = arr[hi], arr[i + 1]
        pi = i + 1
        if pi - lo < hi - pi:
            quicksort_inplace(arr, lo, pi - 1)
            lo = pi + 1
        else:
            quicksort_inplace(arr, pi + 1, hi)
            hi = pi - 1
    # Insertion sort for small partitions
    for i in range(lo + 1, hi + 1):
        key = arr[i]
        j = i - 1
        while j >= lo and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1
        arr[j + 1] = key


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

    # Timsort (C-implemented) — the right way to sort in Python
    t0 = time.perf_counter()
    arr = original.copy()
    arr.sort()
    t1 = time.perf_counter()
    print(
        f"Sort (stdlib): {(t1 - t0) * 1000:.2f} ms ({N / (t1 - t0):.0f} elements/sec)"
    )

    # In-place quicksort (optimized, but still pure Python)
    t0 = time.perf_counter()
    arr = original.copy()
    quicksort_inplace(arr)
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
