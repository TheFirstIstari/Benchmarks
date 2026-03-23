import time
import os

ITERATIONS = 1000
BLOCK_SIZE = 16


def main():
    print("Python Crypto Benchmark")

    key = bytes(i for i in range(16))
    input_data = bytes([0xAA] * 16)
    output = bytearray(16)

    t0 = time.perf_counter()
    for i in range(ITERATIONS):
        for j in range(BLOCK_SIZE):
            output[j] = input_data[j] ^ key[j]
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000

    print(f"XOR (simulated AES): {total_ms:.2f} ms")

    checksum = 0
    t0 = time.perf_counter()
    for i in range(ITERATIONS * 1000):
        checksum += output[i % 16]
    t1 = time.perf_counter()
    total_ms = (t1 - t0) * 1000
    print(f"Integer ops: {total_ms:.2f} ms (checksum: {checksum})")


if __name__ == "__main__":
    main()
