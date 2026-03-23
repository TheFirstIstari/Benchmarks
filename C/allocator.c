#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <pthread.h>
#include <mach/mach_time.h>

#define BLOCK_SIZE 64
#define LIVE_BLOCKS 1024
#define TOTAL_ALLOCS 10000000
#define NUM_THREADS 4

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

typedef struct {
    void* blocks[LIVE_BLOCKS];
    int top;
} thread_ctx_t;

static void* allocate_and_free(void* arg) {
    thread_ctx_t* ctx = calloc(1, sizeof(thread_ctx_t));
    int per_thread = TOTAL_ALLOCS / NUM_THREADS;
    
    for (int i = 0; i < per_thread; i++) {
        void* p = malloc(BLOCK_SIZE);
        ctx->blocks[ctx->top++] = p;
        if (ctx->top >= LIVE_BLOCKS) {
            for (int j = 0; j < LIVE_BLOCKS / 2; j++) {
                free(ctx->blocks[j]);
            }
            memmove(ctx->blocks, ctx->blocks + LIVE_BLOCKS / 2, 
                    (LIVE_BLOCKS / 2) * sizeof(void*));
            ctx->top -= LIVE_BLOCKS / 2;
        }
    }
    
    for (int i = 0; i < ctx->top; i++) {
        free(ctx->blocks[i]);
    }
    free(ctx);
    return NULL;
}

int main(void) {
    printf("C Memory Allocator (%d threads, %d blocks, %d total allocs)\n", 
           NUM_THREADS, LIVE_BLOCKS, TOTAL_ALLOCS);
    
    pthread_t threads[NUM_THREADS];
    
    uint64_t t0 = now_ns();
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, allocate_and_free, NULL);
    }
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    uint64_t t1 = now_ns();
    
    double ms = (t1 - t0) / 1e6;
    double allocs_per_sec = TOTAL_ALLOCS / (ms / 1000.0);
    
    printf("Total time: %.2f ms\n", ms);
    printf("Allocations/sec: %.0f\n", allocs_per_sec);
    printf("Avg latency: %.2f ns/allocation\n", (t1 - t0) / (double)TOTAL_ALLOCS);
    
    return 0;
}
