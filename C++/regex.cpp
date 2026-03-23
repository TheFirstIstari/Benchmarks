#include <iostream>
#include <string>
#include <chrono>
#include <regex>

using namespace std;
using namespace chrono;

constexpr int ITERATIONS = 1000;
constexpr size_t STR_LEN = 100000;

int main() {
    cout << "C++ Regex Benchmark (" << ITERATIONS << " iterations)\n";
    
    string test_str(STR_LEN, 'a');
    for (size_t i = 0; i < STR_LEN; i++) {
        test_str[i] = 'a' + (i % 26);
    }
    
    regex re_find("xyz");
    auto t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        regex_search(test_str, re_find);
    }
    auto t1 = high_resolution_clock::now();
    double ms = duration<double, milli>(t1 - t0).count();
    cout << "Find: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    regex re_count("[aeiou]");
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        auto words_begin = sregex_iterator(test_str.begin(), test_str.end(), re_count);
        auto words_end = sregex_iterator();
        int count = distance(words_begin, words_end);
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Count: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    regex re_email("[a-z]+@[a-z]+\\.[a-z]+");
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        regex_search(test_str.substr(0, 100), re_email);
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Email match: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    regex re_complex("[0-9]{3}-[0-9]{3}-[0-9]{4}");
    t0 = high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        regex_search(test_str, re_complex);
    }
    t1 = high_resolution_clock::now();
    ms = duration<double, milli>(t1 - t0).count();
    cout << "Complex pattern: " << ms << " ms (" << (ITERATIONS / (ms / 1000.0)) << " ops/sec)\n";
    
    return 0;
}
