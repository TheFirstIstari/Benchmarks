import socket
import time
import os
import threading

ITERATIONS = 100
BUFFER_SIZE = 65536


def main():
    print("Python Network Benchmark")

    r, w = os.pipe()

    buffer = b"A" * BUFFER_SIZE

    t0 = time.perf_counter()
    for _ in range(ITERATIONS):
        os.write(w, buffer)
        data = os.read(r, BUFFER_SIZE)
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000

    throughput = (ITERATIONS * BUFFER_SIZE * 2) / (total_ms / 1000) / 1e9
    print(f"Pipe Throughput: {throughput:.2f} GB/s ({total_ms:.2f} ms)")

    import shutil

    data = bytearray(b"A" * 8192)
    dest = bytearray(8192)

    t0 = time.perf_counter()
    for _ in range(ITERATIONS * 10):
        dest[:] = data
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000

    throughput = (ITERATIONS * 10 * 8192) / (total_ms / 1000) / 1e9
    print(f"Memcpy 8K: {throughput:.2f} GB/s ({total_ms:.2f} ms)")

    os.close(r)
    os.close(w)


if __name__ == "__main__":
    main()
