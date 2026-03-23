#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mach/mach_time.h>

#define STR_LEN 1000
#define ITERATIONS 100000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

static inline uint32_t hash_djb2(const char* str) {
    uint32_t hash = 5381;
    int c;
    while ((c = *str++)) {
        hash = ((hash << 5) + hash) + c;
    }
    return hash;
}

static void reverse_string(char* str) {
    size_t len = strlen(str);
    for (size_t i = 0; i < len / 2; i++) {
        char tmp = str[i];
        str[i] = str[len - 1 - i];
        str[len - 1 - i] = tmp;
    }
}

static size_t count_splits(const char* str, char delimiter) {
    size_t count = 0;
    const char* p = str;
    while (*p) {
        if (*p == delimiter) count++;
        p++;
    }
    return count + 1;
}

int main(void) {
    printf("C String Operations (%d iterations)\n", ITERATIONS);
    
    char* test_str = malloc(STR_LEN + 1);
    for (int i = 0; i < STR_LEN; i++) {
        test_str[i] = (char)('a' + (i % 26));
    }
    test_str[STR_LEN] = '\0';
    
    uint64_t t0, t1;
    double total_ms = 0;
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        hash_djb2(test_str);
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Hash: %.2f ms (%.0f ops/sec)\n", 
           total_ms, ITERATIONS / (total_ms / 1000.0));
    
    char* copy = malloc(STR_LEN + 1);
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        memcpy(copy, test_str, STR_LEN);
        reverse_string(copy);
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Reverse: %.2f ms (%.0f ops/sec)\n", 
           total_ms, ITERATIONS / (total_ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        count_splits(test_str, ',');
    }
    t1 = now_ns();
    total_ms = (t1 - t0) / 1e6;
    printf("Split: %.2f ms (%.0f ops/sec)\n", 
           total_ms, ITERATIONS / (total_ms / 1000.0));
    
    free(test_str);
    free(copy);
    
    return 0;
}
