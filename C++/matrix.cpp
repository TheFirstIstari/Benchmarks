#include <iostream>
#include <vector>
#include <chrono>
#include <random>
#include <numeric>

using namespace std;
using namespace chrono;

constexpr int N = 2000;
constexpr int ITERATIONS = 5;

void matmul(vector<double>& C, const vector<double>& A, const vector<double>& B, int n) {
    for (int i = 0; i < n; i++) {
        for (int k = 0; k < n; k++) {
            double aik = A[i * n + k];
            for (int j = 0; j < n; j++) {
                C[i * n + j] += aik * B[k * n + j];
            }
        }
    }
}

void transpose(vector<double>& T, const vector<double>& M, int n) {
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            T[j * n + i] = M[i * n + j];
        }
    }
}

void add_matrices(vector<double>& C, const vector<double>& A, const vector<double>& B, int n) {
    for (int i = 0; i < n * n; i++) {
        C[i] = A[i] + B[i];
    }
}

int main() {
    vector<double> A(N * N), B(N * N), C(N * N), T(N * N);
    mt19937 rng(42);
    uniform_real_distribution<double> dist(0.0, 1.0);
    for (auto& x : A) x = dist(rng);
    for (auto& x : B) x = dist(rng);
    
    cout << "C++ Matrix Benchmark (" << N << "x" << N << ", " << ITERATIONS << " iterations)\n";
    
    double total_ms = 0;
    double checksum = 0;
    
    for (int iter = 0; iter < ITERATIONS; iter++) {
        fill(C.begin(), C.end(), 0.0);
        
        auto t0 = high_resolution_clock::now();
        matmul(C, A, B, N);
        auto t1 = high_resolution_clock::now();
        
        total_ms += duration<double, milli>(t1 - t0).count();
        for (const auto& x : C) checksum += x;
    }
    
    double avg_ms = total_ms / ITERATIONS;
    double ops = (double)N * N * N * 2;
    double gflops = (ops * ITERATIONS) / (total_ms / 1000.0) / 1e9;
    
    cout << "Multiply: " << avg_ms << " ms (" << gflops << " GFLOPS)\n";
    
    volatile double sink = 0;
    auto t0 = high_resolution_clock::now();
    for (int iter = 0; iter < ITERATIONS; iter++) {
        transpose(T, A, N);
        for (const auto& x : T) checksum += x;
        sink += checksum;
    }
    auto t1 = high_resolution_clock::now();
    avg_ms = duration<double, milli>(t1 - t0).count() / ITERATIONS;
    cout << "Transpose: " << avg_ms << " ms\n";
    
    t0 = high_resolution_clock::now();
    for (int iter = 0; iter < ITERATIONS; iter++) {
        add_matrices(C, A, B, N);
    }
    t1 = high_resolution_clock::now();
    avg_ms = duration<double, milli>(t1 - t0).count() / ITERATIONS;
    cout << "Add: " << avg_ms << " ms\n";
    
    return 0;
}
