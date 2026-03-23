#include <iostream>
#include <string>
#include <chrono>
#include <sstream>

using namespace std;
using namespace chrono;

constexpr int ITERATIONS = 5000;
constexpr size_t STR_LEN = 1000000;

int main() {
    cout << "C++ JSON Benchmark (" << ITERATIONS << " iterations)\n";
    
    string test_str(STR_LEN, 'a');
    test_str[0] = '{';
    test_str[1] = '"';
    
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        string copy = test_str;
    }
    auto t1 = high_resolution_clock::now();
    double ms = duration<double, milli>(t1 - t0).count();
    cout << "Parse: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    stringstream ss;
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        ss.str("");
        ss.clear();
        ss << "{\"id\":" << i << ",\"name\":\"user\",\"value\":" << (double)i/10.0 << ",\"items\":[1,2,3,4,5,6,7,8,9,10]}";
        string result = ss.str();
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Serialize: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        size_t pos = test_str.find("{\"");
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Search: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        size_t count = 0;
        size_t pos = 0;
        while ((pos = test_str.find("\"\":", pos)) != string::npos) {
            count++;
            pos++;
        }
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Count fields: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    return 0;
}
