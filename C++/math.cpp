#include <iostream>
#include <cmath>
#include <chrono>

using namespace std;
using namespace chrono;

constexpr int ITERATIONS = 10000000;

int main() {
    cout << "C++ Math Benchmark (" << ITERATIONS << " iterations)\n";
    
    volatile double result = 0;
    auto t0 = high_resolution_clock::now();
    for (int i = 1; i < ITERATIONS; i++) {
        result += sqrt((double)i) + sin((double)i) + cos((double)i);
    }
    auto t1 = high_resolution_clock::now();
    double ms = duration<double, milli>(t1 - t0).count();
    cout << "Trig functions: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    if (result == 0) cout << "";
    
    result = 0;
    t0 = high_resolution_clock::now();
    for (int i = 1; i < ITERATIONS; i++) {
        result += log((double)i) + exp((double)(i % 10));
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Exp/log functions: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    if (result == 0) cout << "";
    
    result = 0;
    t0 = high_resolution_clock::now();
    for (int i = 1; i < ITERATIONS; i++) {
        double x = (double)i;
        result += x * x + x * x * x + sqrt(x * x + 1.0);
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Arithmetic: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    if (result == 0) cout << "";
    
    return 0;
}
