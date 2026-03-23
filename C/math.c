#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <mach/mach_time.h>

#define ITERATIONS 10000000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

int main(void) {
    printf("C Math Benchmark (%d iterations)\n", ITERATIONS);
    
    uint64_t t0, t1;
    volatile double result = 0;
    
    t0 = now_ns();
    for (int i = 1; i < ITERATIONS; i++) {
        result += sqrt((double)i) + sin((double)i) + cos((double)i);
    }
    t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("Trig functions: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    if (result == 0) printf(""); // prevent dead code elimination
    
    result = 0;
    t0 = now_ns();
    for (int i = 1; i < ITERATIONS; i++) {
        result += log((double)i) + exp((double)(i % 10));
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Exp/log functions: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    if (result == 0) printf("");
    
    result = 0;
    t0 = now_ns();
    for (int i = 1; i < ITERATIONS; i++) {
        double x = (double)i;
        result += x * x + x * x * x + sqrt(x * x + 1.0);
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Arithmetic: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    if (result == 0) printf("");
    
    return 0;
}
