#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mach/mach_time.h>

#define N 5000000
#define ITERATIONS 5
#define INSERTION_CUTOFF 32

static mach_timebase_info_data_t tb;

static inline uint64_t now_ns(void) {
    if (tb.denom == 0) mach_timebase_info(&tb);
    return (mach_absolute_time() * tb.numer) / tb.denom;
}

static int cmp_int(const void* a, const void* b) {
    return (*(int*)a) - (*(int*)b);
}

// Insertion sort for small partitions
static inline void insertion_sort(int* restrict arr, int lo, int hi) {
    for (int i = lo + 1; i <= hi; i++) {
        int key = arr[i];
        int j = i - 1;
        while (j >= lo && arr[j] > key) {
            arr[j + 1] = arr[j];
            j--;
        }
        arr[j + 1] = key;
    }
}

static inline void swap(int* a, int* b) {
    int t = *a; *a = *b; *b = t;
}

// Median-of-three pivot selection
static inline int median3(int* arr, int lo, int hi) {
    int mid = lo + (hi - lo) / 2;
    if (arr[lo] > arr[mid]) swap(&arr[lo], &arr[mid]);
    if (arr[lo] > arr[hi]) swap(&arr[lo], &arr[hi]);
    if (arr[mid] > arr[hi]) swap(&arr[mid], &arr[hi]);
    return mid;
}

static void quicksort_opt(int* arr, int lo, int hi) {
    while (hi - lo > INSERTION_CUTOFF) {
        int pivot_idx = median3(arr, lo, hi);
        swap(&arr[pivot_idx], &arr[hi]);
        int pivot = arr[hi];
        int i = lo - 1;
        for (int j = lo; j < hi; j++) {
            if (arr[j] <= pivot) {
                i++;
                swap(&arr[i], &arr[j]);
            }
        }
        swap(&arr[i + 1], &arr[hi]);
        int pi = i + 1;
        // Tail recursion elimination: recurse on smaller side
        if (pi - lo < hi - pi) {
            quicksort_opt(arr, lo, pi - 1);
            lo = pi + 1;
        } else {
            quicksort_opt(arr, pi + 1, hi);
            hi = pi - 1;
        }
    }
    if (hi > lo) insertion_sort(arr, lo, hi);
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

static void mergesort_arr(int* restrict arr, int* restrict temp, int left, int right) {
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

// Radix sort (LSD) for 32-bit integers — O(n) for fixed-width keys
static void radix_sort(int* restrict arr, int* restrict temp, int n) {
    int* src = arr;
    int* dst = temp;
    for (int shift = 0; shift < 32; shift += 8) {
        int count[256] = {0};
        for (int i = 0; i < n; i++) {
            unsigned int val = (unsigned int)(src[i]) >> shift;
            count[val & 0xFF]++;
        }
        // Handle sign bit for last byte
        if (shift == 24) {
            // Negatives need to come before positives
            // Flip sign bit: XOR with 0x80 on the top byte
            int neg[256] = {0};
            int pos[256] = {0};
            for (int i = 128; i < 256; i++) neg[i - 128] = count[i];
            for (int i = 0; i < 128; i++) pos[i] = count[i];
            int idx = 0;
            for (int i = 128; i < 256; i++) {
                count[idx] = neg[i - 128];
                idx++;
            }
            for (int i = 0; i < 128; i++) {
                count[idx] = pos[i];
                idx++;
            }
            // Redo counting with sign-flipped key
            memset(count, 0, sizeof(count));
            for (int i = 0; i < n; i++) {
                unsigned char key = (unsigned char)((src[i] >> 24) ^ 0x80);
                count[key]++;
            }
        }
        // Prefix sum
        int prefix[256];
        int sum = 0;
        for (int i = 0; i < 256; i++) {
            prefix[i] = sum;
            sum += count[i];
        }
        // Scatter
        for (int i = 0; i < n; i++) {
            unsigned char key;
            if (shift == 24) {
                key = (unsigned char)((src[i] >> 24) ^ 0x80);
            } else {
                key = (unsigned char)((unsigned int)(src[i]) >> shift);
            }
            dst[prefix[key]++] = src[i];
        }
        // Swap src/dst
        int* tmp = src; src = dst; dst = tmp;
    }
    // If odd number of passes, result is in temp
    if (src != arr) {
        memcpy(arr, temp, n * sizeof(int));
    }
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
    quicksort_opt(arr, 0, N - 1);
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

    memcpy(arr, original, N * sizeof(int));
    t0 = now_ns();
    radix_sort(arr, temp, N);
    t1 = now_ns();
    printf("Radixsort: %.2f ms (%.0f elements/sec)\n",
           (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));

    free(original);
    free(arr);
    free(temp);

    return 0;
}
