#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdatomic.h>
#include <mach/mach_time.h>

#define N 10000000
#define BRANCH_N 100000000
#define POINTER_N 50000000
#define SIMD_N 50000000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

// 1. Integer arithmetic
static void bench_fibonacci(void) {
    volatile unsigned long result = 0;
    uint64_t t0 = now_ns();
    for (int j = 0; j < N; j++) {
        unsigned long a = 0, b = 1;
        for (int i = 0; i < 50; i++) {
            unsigned long c = a + b;
            a = b; b = c;
        }
        result += a;
    }
    uint64_t t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("cpu_fibonacci: %.2f ms (%.0f ops/sec)\n", ms, N / (ms / 1000.0));
    if (result == 0) printf("");
}

static void bench_prime_sieve(void) {
    int limit = 10000000;
    char* sieve = calloc(limit + 1, 1);
    if (!sieve) return;
    volatile int count = 0;
    uint64_t t0 = now_ns();
    for (int i = 2; i <= limit; i++) {
        if (!sieve[i]) {
            count++;
            if ((long)i * i <= limit)
                for (int j = i * i; j <= limit; j += i)
                    sieve[j] = 1;
        }
    }
    uint64_t t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("cpu_prime_sieve: %.2f ms (%d primes)\n", ms, count);
    free(sieve);
}

static void bench_popcount(void) {
    int* data = malloc(N * sizeof(int));
    for (int i = 0; i < N; i++) data[i] = rand();
    volatile long total = 0;
    uint64_t t0 = now_ns();
    for (int i = 0; i < N; i++)
        total += __builtin_popcount(data[i]);
    uint64_t t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("cpu_popcount: %.2f ms (%.0f ops/sec)\n", ms, N / (ms / 1000.0));
    free(data);
    if (total == 0) printf("");
}

// 2. Branch prediction stress
static void bench_branch_prediction(void) {
    int* sorted = malloc(BRANCH_N * sizeof(int));
    int* unsorted = malloc(BRANCH_N * sizeof(int));
    for (int i = 0; i < BRANCH_N; i++) {
        sorted[i] = i;
        unsorted[i] = rand() % 100;
    }
    
    volatile long sum_sorted = 0;
    uint64_t t0 = now_ns();
    for (int i = 0; i < BRANCH_N; i++) {
        if (sorted[i] > 50000000) sum_sorted += sorted[i];
    }
    uint64_t t1 = now_ns();
    double ms_sorted = (t1 - t0) / 1e6;
    printf("cpu_branch_sorted: %.2f ms\n", ms_sorted);
    
    volatile long sum_unsorted = 0;
    t0 = now_ns();
    for (int i = 0; i < BRANCH_N; i++) {
        if (unsorted[i] > 50) sum_unsorted += unsorted[i];
    }
    t1 = now_ns();
    double ms_unsorted = (t1 - t0) / 1e6;
    printf("cpu_branch_unsorted: %.2f ms (%.1fx slower)\n", ms_unsorted, ms_unsorted / ms_sorted);
    
    free(sorted); free(unsorted);
    if (sum_sorted == 0 || sum_unsorted == 0) printf("");
}

// 3. Memory latency (pointer chasing)
static void bench_memory_latency(void) {
    int* indices = malloc(POINTER_N * sizeof(int));
    for (int i = 0; i < POINTER_N; i++) indices[i] = (i + 1) % POINTER_N;
    // Shuffle
    for (int i = POINTER_N - 1; i > 0; i--) {
        int j = rand() % (i + 1);
        int t = indices[i]; indices[i] = indices[j]; indices[j] = t;
    }
    
    volatile int pos = 0;
    uint64_t t0 = now_ns();
    for (int i = 0; i < POINTER_N; i++)
        pos = indices[pos];
    uint64_t t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("cpu_memory_latency: %.2f ms (%.0f traversals/sec)\n", ms, POINTER_N / (ms / 1000.0));
    free(indices);
    if (pos == 0) printf("");
}

// 4. NEON SIMD vector ops
#ifdef __ARM_NEON
#include <arm_neon.h>
static void bench_simd(void) {
    float* a = aligned_alloc(16, SIMD_N * sizeof(float));
    float* b = aligned_alloc(16, SIMD_N * sizeof(float));
    float* c = aligned_alloc(16, SIMD_N * sizeof(float));
    for (int i = 0; i < SIMD_N; i++) { a[i] = i * 1.0f; b[i] = (i % 100) * 1.0f; }
    
    volatile float result = 0;
    uint64_t t0 = now_ns();
    for (int i = 0; i < SIMD_N; i += 4) {
        float32x4_t va = vld1q_f32(a + i);
        float32x4_t vb = vld1q_f32(b + i);
        float32x4_t vc = vaddq_f32(va, vb);
        vc = vmulq_f32(vc, va);
        vst1q_f32(c + i, vc);
    }
    uint64_t t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    // Sum to prevent DCE
    for (int i = 0; i < 1000; i++) result += c[i * 1000];
    printf("cpu_simd_neon: %.2f ms (%.0f ops/sec)\n", ms, (SIMD_N / 4) / (ms / 1000.0));
    free(a); free(b); free(c);
    if (result == 0) printf("");
}
#else
static void bench_simd(void) {
    float* a = malloc(SIMD_N * sizeof(float));
    float* b = malloc(SIMD_N * sizeof(float));
    float* c = malloc(SIMD_N * sizeof(float));
    for (int i = 0; i < SIMD_N; i++) { a[i] = i * 1.0f; b[i] = (i % 100) * 1.0f; }
    
    volatile float result = 0;
    uint64_t t0 = now_ns();
    for (int i = 0; i < SIMD_N; i++)
        c[i] = (a[i] + b[i]) * a[i];
    uint64_t t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    for (int i = 0; i < 1000; i++) result += c[i * 1000];
    printf("cpu_simd_scalar: %.2f ms (%.0f ops/sec)\n", ms, SIMD_N / (ms / 1000.0));
    free(a); free(b); free(c);
    if (result == 0) printf("");
}
#endif

// 5. Atomic operations
static void bench_atomic(void) {
    atomic_int atomic_val = 0;
    uint64_t t0 = now_ns();
    for (int i = 0; i < N; i++)
        atomic_fetch_add(&atomic_val, 1);
    uint64_t t1 = now_ns();
    double ms_atomic = (t1 - t0) / 1e6;
    printf("cpu_atomic_add: %.2f ms (%.0f ops/sec)\n", ms_atomic, N / (ms_atomic / 1000.0));
    
    volatile int regular_val = 0;
    t0 = now_ns();
    for (int i = 0; i < N; i++)
        regular_val++;
    t1 = now_ns();
    double ms_reg = (t1 - t0) / 1e6;
    printf("cpu_nonatomic_add: %.2f ms (%.0f ops/sec, %.1fx faster)\n", ms_reg, N / (ms_reg / 1000.0), ms_atomic / ms_reg);
    
    if (atomic_val == 0 || regular_val == 0) printf("");
}

int main(void) {
    printf("C CPU Benchmark\n");
    printf("Apple M4 — 10 cores (4P+6E), 16GB unified\n\n");
    
    printf("--- Integer Arithmetic ---\n");
    bench_fibonacci();
    bench_prime_sieve();
    bench_popcount();
    
    printf("\n--- Branch Prediction ---\n");
    bench_branch_prediction();
    
    printf("\n--- Memory Latency ---\n");
    bench_memory_latency();
    
    printf("\n--- SIMD Vectorization ---\n");
    bench_simd();
    
    printf("\n--- Atomic Operations ---\n");
    bench_atomic();
    
    return 0;
}
