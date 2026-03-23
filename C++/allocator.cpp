#include <iostream>
#include <vector>
#include <memory>
#include <chrono>
#include <thread>
#include <mutex>

using namespace std;
using namespace chrono;

constexpr size_t BLOCK_SIZE = 64;
constexpr size_t LIVE_BLOCKS = 1024;
constexpr size_t TOTAL_ALLOCS = 10000000;
constexpr int NUM_THREADS = 4;

struct Block {
    alignas(64) char data[BLOCK_SIZE];
};

struct ThreadCtx {
    vector<unique_ptr<Block>> blocks;
};

mutex g_mutex;

void allocate_and_free(ThreadCtx& ctx) {
    size_t per_thread = TOTAL_ALLOCS / NUM_THREADS;
    for (size_t i = 0; i < per_thread; i++) {
        ctx.blocks.push_back(make_unique<Block>());
        if (ctx.blocks.size() >= LIVE_BLOCKS) {
            size_t half = LIVE_BLOCKS / 2;
            ctx.blocks.erase(ctx.blocks.begin(), ctx.blocks.begin() + half);
        }
    }
    ctx.blocks.clear();
}

int main() {
    cout << "C++ Memory Allocator (" << NUM_THREADS << " threads, " << LIVE_BLOCKS << " blocks, "
         << TOTAL_ALLOCS << " total allocs)\n";
    
    vector<ThreadCtx> contexts(NUM_THREADS);
    vector<thread> threads;
    
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < NUM_THREADS; i++) {
        threads.emplace_back([&ctx = contexts[i]] { allocate_and_free(ctx); });
    }
    for (auto& t : threads) t.join();
    auto t1 = high_resolution_clock::now();
    
    double ms = duration<double, milli>(t1 - t0).count();
    double allocs_per_sec = TOTAL_ALLOCS / (ms / 1000.0);
    
    cout << "Total time: " << ms << " ms\n";
    cout << "Allocations/sec: " << allocs_per_sec << "\n";
    cout << "Avg latency: " << (ms * 1e6) / TOTAL_ALLOCS << " ns/allocation\n";
    
    return 0;
}
