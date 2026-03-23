import time
import numpy as np
import psutil
import torch
from multiprocessing import Pool, cpu_count

# Configuration
LARGE_MATRIX_SIZE = 10000  # For CPU and GPU matrix operations
MEMORY_CHUNK_SIZE = 1024 * 1024 * 100  # 100MB chunks for RAM test
TOTAL_MEMORY_SIZE = 1024 * 1024 * 1000  # 1GB for RAM test
NUM_ITERATIONS = 10  # For repetitive stress testing

# Utility to time tasks
def time_task(func, *args, **kwargs):
    start = time.time()
    result = func(*args, **kwargs)
    end = time.time()
    return result, end - start

# CPU Benchmark: Matrix Multiplication
def cpu_benchmark(matrix_size):
    print("Running CPU Benchmark...")
    a = np.random.rand(matrix_size, matrix_size)
    b = np.random.rand(matrix_size, matrix_size)
    start = time.time()
    c = np.dot(a, b)  # Matrix multiplication
    end = time.time()
    print(f"CPU Benchmark Time: {end - start:.2f} seconds")
    return end - start

# GPU Benchmark: Matrix Multiplication with PyTorch
def gpu_benchmark(matrix_size):
    print("Running GPU Benchmark...")
    device = torch.device("mps" if torch.has_mps else "cpu")  # Use Metal backend on Mac or CPU fallback
    a = torch.rand((matrix_size, matrix_size), device=device)
    b = torch.rand((matrix_size, matrix_size), device=device)
    start = time.time()
    c = torch.matmul(a, b)  # Matrix multiplication
    end = time.time()
    print(f"GPU Benchmark Time: {end - start:.2f} seconds")
    return end - start

# RAM Benchmark: Allocate and Deallocate Large Chunks
def ram_benchmark(chunk_size, total_size):
    print("Running RAM Benchmark...")
    iterations = total_size // chunk_size
    data = []
    start = time.time()
    for _ in range(iterations):
        data.append(bytearray(chunk_size))  # Allocate memory chunk
    del data  # Free memory
    end = time.time()
    print(f"RAM Benchmark Time: {end - start:.2f} seconds")
    return end - start

# Define matrix_multiply globally for parallel CPU benchmark
def matrix_multiply(_):
    matrix_size = LARGE_MATRIX_SIZE // 2
    a = np.random.rand(matrix_size, matrix_size)
    b = np.random.rand(matrix_size, matrix_size)
    return np.dot(a, b)

# Multi-Core CPU Benchmark: Parallel Matrix Multiplications
def parallel_cpu_benchmark(matrix_size, iterations):
    print("Running Multi-Core CPU Benchmark...")
    with Pool(cpu_count()) as pool:
        start = time.time()
        pool.map(matrix_multiply, range(iterations))
        end = time.time()

    print(f"Multi-Core CPU Benchmark Time: {end - start:.2f} seconds")
    return end - start

# Benchmark Suite
def benchmark_suite():
    print("Starting Benchmark Suite...")

    # CPU Benchmark
    cpu_time = cpu_benchmark(LARGE_MATRIX_SIZE)

    # GPU Benchmark
    gpu_time = None
    if torch.has_mps:
        gpu_time = gpu_benchmark(LARGE_MATRIX_SIZE)
    else:
        print("GPU Benchmark skipped: No GPU support detected.")

    # RAM Benchmark
    ram_time = ram_benchmark(MEMORY_CHUNK_SIZE, TOTAL_MEMORY_SIZE)

    # Multi-Core CPU Benchmark
    multi_core_cpu_time = parallel_cpu_benchmark(LARGE_MATRIX_SIZE // 2, NUM_ITERATIONS)

    # Resource Utilization
    cpu_usage = psutil.cpu_percent(interval=1)
    memory_info = psutil.virtual_memory()

    # Summary
    print("\nBenchmark Results:")
    print(f"Single-Core CPU Time: {cpu_time:.2f} seconds")
    if gpu_time is not None:
        print(f"GPU Time: {gpu_time:.2f} seconds")
    print(f"RAM Time: {ram_time:.2f} seconds")
    print(f"Multi-Core CPU Time: {multi_core_cpu_time:.2f} seconds")
    print(f"CPU Usage: {cpu_usage}%")
    print(f"Memory Usage: {memory_info.percent}%")

if __name__ == "__main__":
    benchmark_suite()