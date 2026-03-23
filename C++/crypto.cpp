#include <iostream>
#include <vector>
#include <chrono>

#define ITERATIONS 1000
#define BLOCK_SIZE 16

using namespace std;
using namespace chrono;

int main() {
    cout << "C++ Crypto Benchmark" << endl;
    
    vector<unsigned char> key(16), input(16, 0xAA), output(16);
    for (int i = 0; i < 16; i++) key[i] = i;
    
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        for (int j = 0; j < BLOCK_SIZE; j++) {
            output[j] = input[j] ^ key[j];
        }
    }
    auto t1 = high_resolution_clock::now();
    double total_ms = duration<double, milli>(t1 - t0).count();
    
    cout << "XOR (simulated AES): " << total_ms << " ms" << endl;
    
    int checksum = 0;
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS * 1000; i++) {
        checksum += output[i % 16];
    }
    t1 = high_resolution_clock::now();
    total_ms = duration<double, milli>(t1 - t0).count();
    cout << "Integer ops: " << total_ms << " ms (checksum: " << checksum << ")" << endl;
    
    return 0;
}
