#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mach/mach_time.h>

#define N 2000
#define ITERATIONS 5

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

void matmul(double* C, const double* A, const double* B, int n) {
    for (int i = 0; i < n; i++) {
        for (int k = 0; k < n; k++) {
            double aik = A[i * n + k];
            for (int j = 0; j < n; j++) {
                C[i * n + j] += aik * B[k * n + j];
            }
        }
    }
}

void transpose(double* T, const double* M, int n) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            T[j * n + i] = M[i * n + j];
        }
    }
}

void add_matrices(double* C, const double* A, const double* B, int n) {
    for (int i = 0; i < n * n; i++) {
        C[i] = A[i] + B[i];
    }
}

int main(void) {
    double* A = malloc(N * N * sizeof(double));
    double* B = malloc(N * N * sizeof(double));
    double* C = calloc(N * N, sizeof(double));
    double* T = malloc(N * N * sizeof(double));
    
    srand(42);
    for (int i = 0; i < N * N; i++) {
        A[i] = (double)rand() / RAND_MAX;
        B[i] = (double)rand() / RAND_MAX;
    }
    
    printf("C Matrix Benchmark (%dx%d, %d iterations)\n", N, N, ITERATIONS);
    
    uint64_t total_ns = 0;
    double checksum = 0;
    
    for (int iter = 0; iter < ITERATIONS; iter++) {
        memset(C, 0, N * N * sizeof(double));
        
        uint64_t t0 = now_ns();
        matmul(C, A, B, N);
        uint64_t t1 = now_ns();
        
        total_ns += t1 - t0;
        
        for (int i = 0; i < N * N; i++) checksum += C[i];
    }
    
    double avg_ms = (total_ns / ITERATIONS) / 1e6;
    double ops = (double)N * N * N * 2;
    double gflops = (ops * ITERATIONS) / (total_ns / 1e9) / 1e9;
    
    printf("Multiply: %.2f ms (%.2f GFLOPS)\n", avg_ms, gflops);
    
    volatile double sink = 0;
    uint64_t t0 = now_ns();
    for (int iter = 0; iter < ITERATIONS; iter++) {
        transpose(T, A, N);
        for (int i = 0; i < N * N; i++) checksum += T[i];
        sink += checksum;
    }
    uint64_t t1 = now_ns();
    avg_ms = (t1 - t0) / ITERATIONS / 1e6;
    printf("Transpose: %.2f ms\n", avg_ms);
    
    t0 = now_ns();
    for (int iter = 0; iter < ITERATIONS; iter++) {
        add_matrices(C, A, B, N);
    }
    t1 = now_ns();
    avg_ms = (t1 - t0) / ITERATIONS / 1e6;
    printf("Add: %.2f ms\n", avg_ms);
    
    free(A);
    free(B);
    free(C);
    free(T);
    
    return 0;
}
