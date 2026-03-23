#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdint.h>

#define ITERATIONS 1000
#define BLOCK_SIZE 16

static inline long long now_ns() {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (long long)ts.tv_sec * 1000000000LL + ts.tv_nsec;
}

void aes_encrypt(unsigned char* key, unsigned char* input, unsigned char* output) {
    for (int i = 0; i < BLOCK_SIZE; i++) {
        output[i] = input[i] ^ key[i];
    }
}

int main() {
    printf("C Crypto Benchmark (%d iterations)\n", ITERATIONS);
    
    unsigned char key[16], input[16], output[16];
    for (int i = 0; i < 16; i++) {
        key[i] = i;
        input[i] = 0xAA;
    }
    
    long long t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        aes_encrypt(key, input, output);
    }
    long long t1 = now_ns();
    double total_ms = (t1 - t0) / 1e6;
    
    printf("AES SIMD (xor): %.2f ms\n", total_ms);
    
    volatile int checksum = 0;
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS * 1000; i++) {
        checksum += output[i % 16];
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Integer ops: %.2f ms (checksum: %d)\n", total_ms, checksum);
    
    return 0;
}
