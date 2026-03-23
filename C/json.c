#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mach/mach_time.h>

#define ITERATIONS 5000
#define STR_LEN 1000000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

int main(void) {
    printf("C JSON Benchmark (%d iterations)\n", ITERATIONS);
    
    char* test_str = malloc(STR_LEN);
    for (int i = 0; i < STR_LEN; i++) {
        test_str[i] = 'a';
    }
    test_str[STR_LEN - 1] = '\0';
    
    uint64_t t0, t1;
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        char* copy = malloc(STR_LEN);
        memcpy(copy, test_str, STR_LEN);
        free(copy);
    }
    t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("Parse: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    char output[4096];
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        snprintf(output, sizeof(output), 
            "{\"id\":%d,\"name\":\"user\",\"value\":%.2f,\"items\":[1,2,3,4,5,6,7,8,9,10]}",
            i, (double)i / 10.0);
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Serialize: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        for (int j = 0; j < STR_LEN - 10; j++) {
            if (test_str[j] == '{' && test_str[j+1] == '"') break;
        }
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Search: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        int count = 0;
        for (int j = 0; j < STR_LEN - 1; j++) {
            if (test_str[j] == '"' && test_str[j+1] == ':') count++;
        }
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Count fields: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    free(test_str);
    return 0;
}
