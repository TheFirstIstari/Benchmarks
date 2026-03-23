#include <iostream>
#include <string>
#include <chrono>

using namespace std;
using namespace chrono;

constexpr int ITERATIONS = 1000000;
constexpr size_t STR_LEN = 1000;

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

int main() {
    cout << "C++ Hashing Benchmark (" << ITERATIONS << " iterations)\n";
    
    string data(STR_LEN, 'a');
    volatile unsigned int result = 0;
    
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        result += hash_sdbm(data.c_str(), STR_LEN);
    }
    auto t1 = high_resolution_clock::now();
    double ms = duration<double, milli>(t1 - t0).count();
    cout << "SDBM: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        result += hash_djb2(data.c_str(), STR_LEN);
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "DJB2: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        result += hash_fnv(data.c_str(), STR_LEN);
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "FNV: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    if (result == 0) cout << "";
    return 0;
}
