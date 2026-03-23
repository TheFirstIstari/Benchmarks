#!/usr/bin/env python3
import json
import time

ITERATIONS = 5000


def main():
    print(f"Python JSON Benchmark ({ITERATIONS} iterations)")

    test_data = {
        "name": "test",
        "age": 42,
        "active": True,
        "scores": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        "address": {"street": "Main", "city": "NYC"},
        "tags": ["a", "b", "c", "d", "e"],
    }
    test_json = json.dumps(test_data)
    large_str = '{"data": "' + "x" * 1000000 + '"}'

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        data = json.loads(test_json)
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Parse: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for i in range(ITERATIONS):
        result = json.dumps(
            {"id": i, "name": "user", "value": i / 10.0, "items": list(range(10))}
        )
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Serialize: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        pos = large_str.find('{"')
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Search: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        count = large_str.count('":')
    t1 = time.perf_counter()
    ms = (t1 - t0) * 1000
    print(f"Count fields: {ms:.2f} ms ({ITERATIONS / (ms / 1000):.0f} ops/sec)")


if __name__ == "__main__":
    main()
