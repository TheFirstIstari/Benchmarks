import json
import time

ITERATIONS = 100
PAYLOAD_SIZE = 10000


def create_payload():
    return {
        "users": [
            {
                "id": i,
                "name": f"user{i}",
                "email": f"user{i}@example.com",
                "active": True,
            }
            for i in range(100)
        ],
        "data": "A" * PAYLOAD_SIZE,
    }


def main():
    print("Python JSON Benchmark")

    data = create_payload()
    json_str = json.dumps(data)

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        parsed = json.loads(json_str)
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000
    print(
        f"json.loads: {total_ms:.2f} ms ({ITERATIONS / (total_ms / 1000):.0f} ops/sec)"
    )

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        s = json.dumps(data)
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000
    print(
        f"json.dumps: {total_ms:.2f} ms ({ITERATIONS / (total_ms / 1000):.0f} ops/sec)"
    )

    try:
        import orjson

        t0 = time.perf_counter()
        for _ in range(ITERATIONS):
            parsed = orjson.loads(json_str)
        t1 = time.perf_counter()
        total_ms = (t1 - t0) * 1000
        print(
            f"orjson.loads: {total_ms:.2f} ms ({ITERATIONS / (total_ms / 1000):.0f} ops/sec)"
        )
    except ImportError:
        pass

    try:
        import ujson

        t0 = time.perf_counter()
        for _ in range(ITERATIONS):
            parsed = ujson.loads(json_str)
        t1 = time.perf_counter()
        total_ms = (t1 - t0) * 1000
        print(
            f"ujson.loads: {total_ms:.2f} ms ({ITERATIONS / (total_ms / 1000):.0f} ops/sec)"
        )
    except ImportError:
        pass


if __name__ == "__main__":
    main()
