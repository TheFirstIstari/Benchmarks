#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mach/mach_time.h>

#define ITERATIONS 1000
#define FILE_SIZE 5000000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

int main(void) {
    printf("C File I/O Benchmark (%d iterations)\n", ITERATIONS);
    
    char* data = malloc(FILE_SIZE);
    for (int i = 0; i < FILE_SIZE; i++) {
        data[i] = 'a' + (i % 26);
    }
    data[FILE_SIZE - 1] = '\0';
    
    remove("/tmp/bench_io.dat");
    
    uint64_t t0, t1;
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        FILE* f = fopen("/tmp/bench_io.dat", "wb");
        fwrite(data, 1, FILE_SIZE, f);
        fclose(f);
    }
    t1 = now_ns();
    double ms = (t1 - t0) / 1e6;
    printf("Write: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        FILE* f = fopen("/tmp/bench_io.dat", "rb");
        fread(data, 1, FILE_SIZE, f);
        fclose(f);
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Read: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        FILE* f = fopen("/tmp/bench_io.dat", "r");
        char line[256];
        int count = 0;
        while (fgets(line, sizeof(line), f) != NULL) count++;
        fclose(f);
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Read lines: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    t0 = now_ns();
    for (int i = 0; i < ITERATIONS; i++) {
        FILE* f = fopen("/tmp/bench_io.dat", "rb");
        fseek(f, FILE_SIZE / 2, SEEK_SET);
        fread(data, 1, 1024, f);
        fclose(f);
    }
    t1 = now_ns();
    ms = (t1 - t0) / 1e6;
    printf("Random access: %.2f ms (%.0f ops/sec)\n", ms, ITERATIONS / (ms / 1000.0));
    
    remove("/tmp/bench_io.dat");
    free(data);
    return 0;
}
