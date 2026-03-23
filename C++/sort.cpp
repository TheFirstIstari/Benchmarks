#include <iostream>
#include <vector>
#include <algorithm>
#include <random>
#include <chrono>

using namespace std;
using namespace chrono;

constexpr size_t N = 5000000;
constexpr int ITERATIONS = 5;

template<typename T>
void quicksort(vector<T>& arr, int low, int high) {
    if (low >= high) return;
    T pivot = arr[high];
    int i = low - 1;
    for (int j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            i++;
            swap(arr[i], arr[j]);
        }
    }
    swap(arr[i + 1], arr[high]);
    quicksort(arr, low, i);
    quicksort(arr, i + 2, high);
}

template<typename T>
void heapsort(vector<T>& arr) {
    make_heap(arr.begin(), arr.end());
    sort_heap(arr.begin(), arr.end());
}

template<typename T>
void mergesort_vec(vector<T>& arr, vector<T>& temp, size_t left, size_t right) {
    if (left >= right) return;
    size_t mid = left + (right - left) / 2;
    mergesort_vec(arr, temp, left, mid);
    mergesort_vec(arr, temp, mid + 1, right);
    size_t i = left, j = mid + 1, k = left;
    while (i <= mid && j <= right) {
        if (arr[i] <= arr[j]) temp[k++] = arr[i++];
        else temp[k++] = arr[j++];
    }
    while (i <= mid) temp[k++] = arr[i++];
    while (j <= right) temp[k++] = arr[j++];
    for (i = left; i <= right; i++) arr[i] = temp[i];
}

template<typename T>
void my_mergesort(vector<T>& arr) {
    vector<T> temp(arr.size());
    mergesort_vec(arr, temp, 0, arr.size() - 1);
}

int main() {
    cout << "C++ Sort Benchmark (N=" << N << ", " << ITERATIONS << " iterations)\n";
    
    vector<int> original(N);
    mt19937 rng(42);
    for (auto& x : original) x = rng();
    
    vector<int> arr(N);
    
    arr = original;
    auto t0 = high_resolution_clock::now();
    sort(arr.begin(), arr.end());
    auto t1 = high_resolution_clock::now();
    cout << "Sort (stdlib): " << duration<double, milli>(t1 - t0).count() << " ms ("
         << (N / (duration<double>(t1 - t0).count())) << " elements/sec)\n";
    
    arr = original;
    t0 = high_resolution_clock::now();
    quicksort(arr, 0, arr.size() - 1);
    t1 = high_resolution_clock::now();
    cout << "Quicksort: " << duration<double, milli>(t1 - t0).count() << " ms ("
         << (N / (duration<double>(t1 - t0).count())) << " elements/sec)\n";
    
    arr = original;
    t0 = high_resolution_clock::now();
    heapsort(arr);
    t1 = high_resolution_clock::now();
    cout << "Heapsort: " << duration<double, milli>(t1 - t0).count() << " ms ("
         << (N / (duration<double>(t1 - t0).count())) << " elements/sec)\n";
    
    arr = original;
    t0 = high_resolution_clock::now();
    my_mergesort(arr);
    t1 = high_resolution_clock::now();
    cout << "Mergesort: " << duration<double, milli>(t1 - t0).count() << " ms ("
         << (N / (duration<double>(t1 - t0).count())) << " elements/sec)\n";
    
    return 0;
}
