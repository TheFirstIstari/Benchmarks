#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdatomic.h>
#include <mach/mach_time.h>

#define NUM_THREADS 8
#define OPS_PER_THREAD 10000000

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

static _Atomic long counter = 0;

static void* atomic_increment(void* arg) {
    for (long i = 0; i < OPS_PER_THREAD; i++) {
        atomic_fetch_add(&counter, 1);
    }
    return NULL;
}

static void* parallel_sum(void* arg) {
    long thread_id = *(long*)arg;
    long per_thread = OPS_PER_THREAD;
    long start = thread_id * per_thread;
    long end = start + per_thread;
    long local_sum = 0;
    for (long i = start; i < end; i++) {
        local_sum += i * i;
    }
    return (void*)local_sum;
}

static void* thread_pool_task(void* arg) {
    long thread_id = *(long*)arg;
    long per_thread = OPS_PER_THREAD;
    long start = thread_id * per_thread;
    long local_sum = 0;
    for (long i = start; i < start + per_thread; i++) {
        local_sum += i * i;
    }
    return (void*)local_sum;
}

int main(void) {
    printf("C Concurrency Benchmark (%d threads, %d ops/thread)\n", 
           NUM_THREADS, OPS_PER_THREAD);
    
    pthread_t threads[NUM_THREADS];
    uint64_t t0, t1;
    long total_ops = (long)NUM_THREADS * OPS_PER_THREAD;
    
    counter = 0;
    t0 = now_ns();
    for (int i = 0; i < NUM_THREADS; i++) pthread_create(&threads[i], NULL, atomic_increment, NULL);
    for (int i = 0; i < NUM_THREADS; i++) pthread_join(threads[i], NULL);
    t1 = now_ns();
    printf("Atomic: %.2f ms (%.0f ops/sec)\n", 
           (t1 - t0) / 1e6, total_ops / ((t1 - t0) / 1e9));
    
    long results[NUM_THREADS];
    t0 = now_ns();
    for (long i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, parallel_sum, &i);
    }
    for (int i = 0; i < NUM_THREADS; i++) pthread_join(threads[i], (void**)&results[i]);
    t1 = now_ns();
    printf("Parallel sum: %.2f ms (%.0f ops/sec)\n", 
           (t1 - t0) / 1e6, total_ops / ((t1 - t0) / 1e9));
    
    t0 = now_ns();
    for (long i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, thread_pool_task, &i);
    }
    for (int i = 0; i < NUM_THREADS; i++) pthread_join(threads[i], (void**)&results[i]);
    t1 = now_ns();
    printf("Thread pool: %.2f ms (%.0f ops/sec)\n", 
           (t1 - t0) / 1e6, total_ops / ((t1 - t0) / 1e9));
    
    return 0;
}
