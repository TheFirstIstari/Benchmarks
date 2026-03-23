#include <iostream>
#include <vector>
#include <chrono>
#include <unistd.h>
#include <string.h>

#define ITERATIONS 100
#define BUFFER_SIZE 65536

using namespace std;
using namespace chrono;

int main() {
    cout << "C++ Network Benchmark" << endl;
    
    int fds[2];
    pipe(fds);
    
    vector<char> buffer(BUFFER_SIZE, 'A');
    
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        write(fds[1], buffer.data(), BUFFER_SIZE);
        read(fds[0], buffer.data(), BUFFER_SIZE);
    }
    auto t1 = high_resolution_clock::now();
    double total_ms = duration<double, milli>(t1 - t0).count();
    
    double throughput = (ITERATIONS * BUFFER_SIZE * 2) / (total_ms / 1000.0) / 1e9;
    cout << "Pipe Throughput: " << throughput << " GB/s (" << total_ms << " ms)" << endl;
    
    vector<char> src(8192, 'A');
    vector<char> dst(8192, 0);
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS * 10; i++) {
        memcpy(dst.data(), src.data(), 8192);
    }
    t1 = high_resolution_clock::now();
    total_ms = duration<double, milli>(t1 - t0).count();
    
    throughput = (ITERATIONS * 10 * 8192) / (total_ms / 1000.0) / 1e9;
    cout << "Memcpy 8K: " << throughput << " GB/s (" << total_ms << " ms)" << endl;
    
    close(fds[0]);
    close(fds[1]);
    
    return 0;
}
