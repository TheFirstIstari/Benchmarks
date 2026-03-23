#include <iostream>
#include <fstream>
#include <string>
#include <chrono>

using namespace std;
using namespace chrono;

constexpr int ITERATIONS = 1000;
constexpr size_t FILE_SIZE = 5000000;

int main() {
    cout << "C++ File I/O Benchmark (" << ITERATIONS << " iterations)\n";
    
    string data(FILE_SIZE, 'a');
    for (size_t i = 0; i < FILE_SIZE; i++) {
        data[i] = 'a' + (i % 26);
    }
    
    remove("/tmp/bench_io.dat");
    
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        ofstream f("/tmp/bench_io.dat", ios::binary);
        f.write(data.c_str(), FILE_SIZE);
        f.close();
    }
    auto t1 = high_resolution_clock::now();
    double ms = duration<double, milli>(t1 - t0).count();
    cout << "Write: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    string read_buffer(FILE_SIZE, '\0');
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        ifstream f("/tmp/bench_io.dat", ios::binary);
        f.read(&read_buffer[0], FILE_SIZE);
        f.close();
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Read: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        ifstream f("/tmp/bench_io.dat");
        string line;
        while (getline(f, line)) {}
        f.close();
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Read lines: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        fstream f("/tmp/bench_io.dat", ios::in | ios::binary);
        f.seekg(FILE_SIZE / 2);
        f.read(&read_buffer[0], 1024);
        f.close();
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Random access: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    remove("/tmp/bench_io.dat");
    return 0;
}
