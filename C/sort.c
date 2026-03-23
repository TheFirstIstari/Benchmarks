#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mach/mach_time.h>

#define N 5000000
#define ITERATIONS 5

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

static int cmp_int(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}

static void swap(int* a, int* b) {
    int t = *a; *a = *b; *b = t;
}

static int partition(int* arr, int low, int high) {
    int pivot = arr[high];
    int i = low - 1;
    for (int j = low; j < high; j++) {
        if (arr[j] <= pivot) {
            i++;
            swap(&arr[i], &arr[j]);
        }
    }
    swap(&arr[i + 1], &arr[high]);
    return i + 1;
}

static void quicksort(int* arr, int low, int high) {
    if (low < high) {
        int pi = partition(arr, low, high);
        quicksort(arr, low, pi - 1);
        quicksort(arr, pi + 1, high);
    }
}

static void heapify(int* arr, int n, int i) {
    int largest = i;
    int left = 2 * i + 1;
    int right = 2 * i + 2;
    if (left < n && arr[left] > arr[largest]) largest = left;
    if (right < n && arr[right] > arr[largest]) largest = right;
    if (largest != i) {
        swap(&arr[i], &arr[largest]);
        heapify(arr, n, largest);
    }
}

static void my_heapsort(int* arr, int n) {
    for (int i = n / 2 - 1; i >= 0; i--) heapify(arr, n, i);
    for (int i = n - 1; i > 0; i--) {
        swap(&arr[0], &arr[i]);
        heapify(arr, i, 0);
    }
}

static void mergesort_arr(int* arr, int* temp, int left, int right) {
    if (left >= right) return;
    int mid = left + (right - left) / 2;
    mergesort_arr(arr, temp, left, mid);
    mergesort_arr(arr, temp, mid + 1, right);
    int i = left, j = mid + 1, k = left;
    while (i <= mid && j <= right) {
        if (arr[i] <= arr[j]) temp[k++] = arr[i++];
        else temp[k++] = arr[j++];
    }
    while (i <= mid) temp[k++] = arr[i++];
    while (j <= right) temp[k++] = arr[j++];
    for (i = left; i <= right; i++) arr[i] = temp[i];
}

static void my_mergesort(int* arr, int* temp, int n) {
    mergesort_arr(arr, temp, 0, n - 1);
}

int main(void) {
    printf("C Sort Benchmark (N=%d, %d iterations)\n", N, ITERATIONS);
    
    int* original = malloc(N * sizeof(int));
    int* arr = malloc(N * sizeof(int));
    int* temp = malloc(N * sizeof(int));
    srand(42);
    for (int i = 0; i < N; i++) original[i] = rand();
    
    memcpy(arr, original, N * sizeof(int));
    uint64_t t0 = now_ns();
    qsort(arr, N, sizeof(int), cmp_int);
    uint64_t t1 = now_ns();
    printf("Qsort (stdlib): %.2f ms (%.0f elements/sec)\n",
           (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));
    
    memcpy(arr, original, N * sizeof(int));
    t0 = now_ns();
    quicksort(arr, 0, N - 1);
    t1 = now_ns();
    printf("Quicksort: %.2f ms (%.0f elements/sec)\n",
           (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));
    
    memcpy(arr, original, N * sizeof(int));
    t0 = now_ns();
    my_heapsort(arr, N);
    t1 = now_ns();
    printf("Heapsort: %.2f ms (%.0f elements/sec)\n",
           (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));
    
    memcpy(arr, original, N * sizeof(int));
    t0 = now_ns();
    my_mergesort(arr, temp, N);
    t1 = now_ns();
    printf("Mergesort: %.2f ms (%.0f elements/sec)\n",
           (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));
    
    free(original);
    free(arr);
    free(temp);
    
    return 0;
}
