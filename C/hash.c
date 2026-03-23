#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mach/mach_time.h>

#define ITERATIONS 1000000
#define STR_LEN 1000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

unsigned int hash_sdbm(const char* str, size_t len) {
    unsigned int hash = 0;
    for (size_t i = 0; i < len; i++) {
        hash = str[i] + (hash << 6) + (hash << 16) - hash;
    }
    return hash;
}

unsigned int hash_djb2(const char* str, size_t len) {
    unsigned int hash = 5381;
    for (size_t i = 0; i < len; i++) {
        hash = ((hash << 5) + hash) + str[i];
    }
    return hash;
}

unsigned int hash_fnv(const char* str, size_t len) {
    unsigned int hash = 2166136261u;
    for (size_t i = 0; i < len; i++) {
        hash ^= str[i];
        hash *= 16777619u;
    }
    return hash;
}

int main(void) {
    printf("C Hashing Benchmark (%d iterations)\n", ITERATIONS);
    
    char* data = malloc(STR_LEN);
    memset(data, 'a', STR_LEN);
    volatile unsigned int result = 0;
    uint64_t t0, t1;
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        result += hash_sdbm(data, STR_LEN);
    }
    t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("SDBM: %.2f ms (%.0f ops/sec)\n", 
           ms, ITERATIONS / (ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        result += hash_djb2(data, STR_LEN);
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("DJB2: %.2f ms (%.0f ops/sec)\n", 
           ms, ITERATIONS / (ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        result += hash_fnv(data, STR_LEN);
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("FNV: %.2f ms (%.0f ops/sec)\n", 
           ms, ITERATIONS / (ms / 1000.0));
    
    if (result == 0) printf(""); // prevent optimization
    free(data);
    return 0;
}
