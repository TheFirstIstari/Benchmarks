#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

#define ITERATIONS 100
#define VECTOR_SIZE 1024

static inline long long now_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (long long)ts.tv_sec * 1000000000LL + ts.tv_nsec;
}

int main() {
    printf("C ML Benchmark (%d iterations, size %d)\n", ITERATIONS, VECTOR_SIZE);
    
    float* a = malloc(VECTOR_SIZE * sizeof(float));
    float* b = malloc(VECTOR_SIZE * sizeof(float));
    float* c = malloc(VECTOR_SIZE * sizeof(float));
    
    for (int i = 0; i < VECTOR_SIZE; i++) {
        a[i] = (float)i / VECTOR_SIZE;
        b[i] = (float)(VECTOR_SIZE - i) / VECTOR_SIZE;
        c[i] = 0;
    }
    
    long long t0 = now_ns();
    for (int iter = 0; iter < ITERATIONS; iter++) {
        for (int i = 0; i < VECTOR_SIZE; i++) {
            c[i] = a[i] * b[i];
        }
    }
    long long t1 = now_ns();
    double total_ms = (t1 - t0) / 1e6;
    printf("Element-wise mul: %.2f ms\n", total_ms);
    
    t0 = now_ns();
    float dot = 0;
    for (int iter = 0; iter < ITERATIONS; iter++) {
        dot = 0;
        for (int i = 0; i < VECTOR_SIZE; i++) {
            dot += a[i] * b[i];
        }
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Dot product: %.2f ms (result: %.4f)\n", total_ms, dot);
    
    t0 = now_ns();
    for (int iter = 0; iter < ITERATIONS; iter++) {
        for (int i = 0; i < VECTOR_SIZE; i++) {
            a[i] = a[i] * 0.9f + b[i] * 0.1f;
        }
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Lerp (mix): %.2f ms\n", total_ms);
    
    t0 = now_ns();
    for (int iter = 0; iter < ITERATIONS; iter++) {
        for (int i = 0; i < VECTOR_SIZE; i++) {
            c[i] = 1.0f / (1.0f + a[i]);
        }
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Sigmoid: %.2f ms\n", total_ms);
    
    free(a);
    free(b);
    free(c);
    
    return 0;
}
