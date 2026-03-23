#include <iostream>
#include <string>
#include <algorithm>
#include <chrono>

using namespace std;
using namespace chrono;

constexpr size_t STR_LEN = 1000;
constexpr int ITERATIONS = 100000;

uint32_t hash_djb2(const string& str) {
    uint32_t hash = 5381;
    for (char c : str) hash = ((hash << 5) + hash) + c;
    return hash;
}

size_t count_splits(const string& str, char delimiter) {
    size_t count = 0;
    for (char c : str) if (c == delimiter) count++;
    return count + 1;
}

int main() {
    cout << "C++ String Operations (" << ITERATIONS << " iterations)\n";
    
    string test_str(STR_LEN, 'a');
    for (size_t i = 0; i < STR_LEN; i++) test_str[i] = 'a' + (i % 26);
    
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        hash_djb2(test_str);
    }
    auto t1 = high_resolution_clock::now();
    double ms = duration<double, milli>(t1 - t0).count();
    cout << "Hash: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    string copy = test_str;
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        copy = test_str;
        reverse(copy.begin(), copy.end());
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Reverse: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        count_splits(test_str, ',');
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Split: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    return 0;
}
