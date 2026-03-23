import java.util.*;

public class SortBench {
    static final int N = 5_000_000;
    static final int ITERATIONS = 5;
    
    static void quicksort(int[] arr, int low, int high) {
        if (low >= high) return;
        int pivot = arr[high];
        int i = low - 1;
        for (int j = low; j < high; j++) {
            if (arr[j] <= pivot) {
                i++;
                int temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
        int temp = arr[i + 1];
        arr[i + 1] = arr[high];
        arr[high] = temp;
        quicksort(arr, low, i);
        quicksort(arr, i + 2, high);
    }
    
    public static void main(String[] args) {
        System.out.println("Java Sort Benchmark (N=" + N + ", " + ITERATIONS + " iterations)");
        
        int[] original = new int[N];
        Random rng = new Random(42);
        for (int i = 0; i < N; i++) original[i] = rng.nextInt(1_000_000);
        
        int[] arr = new int[N];
        
        System.arraycopy(original, 0, arr, 0, N);
        long t0 = System.nanoTime();
        Arrays.sort(arr);
        long t1 = System.nanoTime();
        System.out.printf("Sort (stdlib): %.2f ms (%.0f elements/sec)%n",
            (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));
        
        System.arraycopy(original, 0, arr, 0, N);
        t0 = System.nanoTime();
        quicksort(arr, 0, arr.length - 1);
        t1 = System.nanoTime();
        System.out.printf("Quicksort: %.2f ms (%.0f elements/sec)%n",
            (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));
        
        System.arraycopy(original, 0, arr, 0, N);
        t0 = System.nanoTime();
        Arrays.parallelSort(arr);
        t1 = System.nanoTime();
        System.out.printf("ParallelSort: %.2f ms (%.0f elements/sec)%n",
            (t1 - t0) / 1e6, N / ((t1 - t0) / 1e9));
    }
}
