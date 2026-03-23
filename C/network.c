#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

#define ITERATIONS 100
#define BUFFER_SIZE 65536

static inline long long now_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (long long)ts.tv_sec * 1000000000LL + ts.tv_nsec;
}

int main() {
    printf("C Network Benchmark (%d iterations)\n", ITERATIONS);
    
    int fds[2];
    if (pipe(fds) < 0) {
        perror("pipe");
        return 1;
    }
    
    char* buffer = malloc(BUFFER_SIZE);
    memset(buffer, 'A', BUFFER_SIZE);
    
    long long t0, t1;
    double total_ms;
    double throughput;
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        write(fds[1], buffer, BUFFER_SIZE);
        read(fds[0], buffer, BUFFER_SIZE);
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    
    throughput = (ITERATIONS * BUFFER_SIZE * 2) / (total_ms / 1000.0) / 1e9;
    printf("Pipe Throughput: %.2f GB/s (%.2f ms)\n", throughput, total_ms);
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        memcpy(buffer + 4096, buffer, 4096);
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    
    throughput = (ITERATIONS * 8192) / (total_ms / 1000.0) / 1e9;
    printf("Memcpy 8K: %.2f GB/s (%.2f ms)\n", throughput, total_ms);
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS * 100; i++) {
        volatile int x = 0;
        x++;
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Loop overhead: %.2f ms\n", total_ms);
    
    free(buffer);
    close(fds[0]);
    close(fds[1]);
    
    return 0;
}
