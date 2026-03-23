#include <iostream>
#include <thread>
#include <atomic>
#include <vector>
#include <chrono>
#include <future>

using namespace std;
using namespace chrono;

constexpr int NUM_THREADS = 8;
constexpr long OPS_PER_THREAD = 10000000;

atomic<long> counter{0};

void atomic_increment() {
    for (long i = 0; i < OPS_PER_THREAD; i++) {
        counter.fetch_add(1);
    }
}

long parallel_sum(long start, long end) {
    long sum = 0;
    for (long i = start; i < end; i++) {
        sum += i * i;
    }
    return sum;
}

long thread_pool_task(long start) {
    long sum = 0;
    for (long i = start; i < start + OPS_PER_THREAD; i++) {
        sum += i * i;
    }
    return sum;
}

int main() {
    cout << "C++ Concurrency Benchmark (" << NUM_THREADS << " threads, "
         << OPS_PER_THREAD << " ops/thread)\n";
    
    long total_ops = (long)NUM_THREADS * OPS_PER_THREAD;
    auto t0 = high_resolution_clock::now();
    vector<thread> threads;
    for (int i = 0; i < NUM_THREADS; i++) threads.emplace_back(atomic_increment);
    for (auto& t : threads) t.join();
    auto t1 = high_resolution_clock::now();
    cout << "Atomic: " << duration<double, milli>(t1 - t0).count() << " ms ("
         << (total_ops / duration<double>(t1 - t0).count()) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    vector<future<long>> futures;
    for (int i = 0; i < NUM_THREADS; i++) {
        futures.push_back(async(launch::async, parallel_sum, i * OPS_PER_THREAD, (i + 1) * OPS_PER_THREAD));
    }
    long sum = 0;
    for (auto& f : futures) sum += f.get();
    t1 = high_resolution_clock::now();
    cout << "Parallel sum: " << duration<double, milli>(t1 - t0).count() << " ms ("
         << (total_ops / duration<double>(t1 - t0).count()) << " ops/sec)\n";
    
    threads.clear();
    t0 = high_resolution_clock::now();
    for (int i = 0; i < NUM_THREADS; i++) {
        threads.emplace_back(thread_pool_task, i * OPS_PER_THREAD);
    }
    for (auto& t : threads) t.join();
    t1 = high_resolution_clock::now();
    cout << "Thread pool: " << duration<double, milli>(t1 - t0).count() << " ms ("
         << (total_ops / duration<double>(t1 - t0).count()) << " ops/sec)\n";
    
    return 0;
}
