import java.util.Random;
import java.util.Arrays;
import java.util.concurrent.atomic.AtomicInteger;

public class CpuBench {
    static final int N = 10_000_000;
    static final int BRANCH_N = 100_000_000;
    static final int POINTER_N = 50_000_000;
    static final int SIMD_N = 50_000_000;
    static final int WARMUP = 3;
    static Random rand = new Random(42);

    public static void main(String[] args) {
        System.out.println("Java CPU Benchmark");
        System.out.println("Apple M4 — 10 cores (4P+6E), 16GB unified\n");

        System.out.println("--- Integer Arithmetic ---");
        benchFibonacci();
        benchPrimeSieve();
        benchPopcount();

        System.out.println("\n--- Branch Prediction ---");
        benchBranchPrediction();

        System.out.println("\n--- Memory Latency ---");
        benchMemoryLatency();

        System.out.println("\n--- SIMD Vectorization ---");
        benchSimd();

        System.out.println("\n--- Atomic Operations ---");
        benchAtomic();
    }

    // 1. Integer arithmetic
    static void benchFibonacci() {
        for (int w = 0; w < WARMUP; w++) {
            long r = 0;
            for (int j = 0; j < N; j++) {
                long a = 0, b = 1;
                for (int i = 0; i < 50; i++) { long c = a + b; a = b; b = c; }
                r += a;
            }
        }

        long result = 0;
        long t0 = System.nanoTime();
        for (int j = 0; j < N; j++) {
            long a = 0, b = 1;
            for (int i = 0; i < 50; i++) { long c = a + b; a = b; b = c; }
            result += a;
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("cpu_fibonacci: %.2f ms (%.0f ops/sec)%n", ms, N / (ms / 1000));
    }

    static void benchPrimeSieve() {
        for (int w = 0; w < WARMUP; w++) {
            boolean[] sieve = new boolean[10_000_001];
            int cnt = 0;
            for (int i = 2; i <= 10_000_000; i++) {
                if (!sieve[i]) {
                    cnt++;
                    if ((long)i * i <= 10_000_000)
                        for (int j = i * i; j <= 10_000_000; j += i) sieve[j] = true;
                }
            }
        }

        boolean[] sieve = new boolean[10_000_001];
        long t0 = System.nanoTime();
        int count = 0;
        for (int i = 2; i <= 10_000_000; i++) {
            if (!sieve[i]) {
                count++;
                if ((long)i * i <= 10_000_000)
                    for (int j = i * i; j <= 10_000_000; j += i) sieve[j] = true;
            }
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("cpu_prime_sieve: %.2f ms (%d primes)%n", ms, count);
    }

    static void benchPopcount() {
        int[] data = new int[N];
        for (int i = 0; i < N; i++) data[i] = rand.nextInt();

        for (int w = 0; w < WARMUP; w++) {
            long r = 0;
            for (int i = 0; i < N; i++) r += Integer.bitCount(data[i]);
        }

        long t0 = System.nanoTime();
        long total = 0;
        for (int i = 0; i < N; i++) total += Integer.bitCount(data[i]);
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("cpu_popcount: %.2f ms (%.0f ops/sec)%n", ms, N / (ms / 1000));
    }

    // 2. Branch prediction
    static void benchBranchPrediction() {
        int[] unsorted = new int[BRANCH_N];
        for (int i = 0; i < BRANCH_N; i++) unsorted[i] = rand.nextInt(100);
        int[] sorted = unsorted.clone();
        Arrays.sort(sorted);

        for (int w = 0; w < WARMUP; w++) {
            long r = 0;
            for (int i = 0; i < BRANCH_N; i++) if (unsorted[i] > 50) r += unsorted[i];
        }

        long t0 = System.nanoTime();
        long sumUnsorted = 0;
        for (int i = 0; i < BRANCH_N; i++) if (unsorted[i] > 50) sumUnsorted += unsorted[i];
        long t1 = System.nanoTime();
        double msUnsorted = (t1 - t0) / 1e6;
        System.out.printf("cpu_branch_unsorted: %.2f ms%n", msUnsorted);

        for (int w = 0; w < WARMUP; w++) {
            long r = 0;
            for (int i = 0; i < BRANCH_N; i++) if (sorted[i] > 50) r += sorted[i];
        }

        t0 = System.nanoTime();
        long sumSorted = 0;
        for (int i = 0; i < BRANCH_N; i++) if (sorted[i] > 50) sumSorted += sorted[i];
        t1 = System.nanoTime();
        double msSorted = (t1 - t0) / 1e6;
        System.out.printf("cpu_branch_sorted: %.2f ms (%.1fx faster)%n", msSorted, msUnsorted / msSorted);
    }

    // 3. Memory latency
    static void benchMemoryLatency() {
        int[] indices = new int[POINTER_N];
        for (int i = 0; i < POINTER_N; i++) indices[i] = (i + 1) % POINTER_N;
        // Fisher-Yates
        for (int i = POINTER_N - 1; i > 0; i--) {
            int j = rand.nextInt(i + 1);
            int t = indices[i]; indices[i] = indices[j]; indices[j] = t;
        }

        for (int w = 0; w < WARMUP; w++) {
            int pos = 0;
            for (int i = 0; i < POINTER_N; i++) pos = indices[pos];
        }

        long t0 = System.nanoTime();
        int pos = 0;
        for (int i = 0; i < POINTER_N; i++) pos = indices[pos];
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("cpu_memory_latency: %.2f ms (%.0f traversals/sec)%n", ms, POINTER_N / (ms / 1000));
    }

    // 4. SIMD — JIT auto-vectorizes simple float loops
    static void benchSimd() {
        float[] a = new float[SIMD_N];
        float[] b = new float[SIMD_N];
        float[] c = new float[SIMD_N];
        for (int i = 0; i < SIMD_N; i++) { a[i] = i; b[i] = (int)(System.nanoTime() % 100); }

        for (int w = 0; w < WARMUP; w++) {
            for (int i = 0; i < SIMD_N; i++) c[i] = (a[i] + b[i]) * a[i];
        }

        long t0 = System.nanoTime();
        for (int i = 0; i < SIMD_N; i++) c[i] = (a[i] + b[i]) * a[i];
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        float total = 0;
        for (int i = 0; i < 1000; i++) total += c[i * 1000];
        System.out.printf("cpu_simd_auto_vec: %.2f ms (%.0f ops/sec)%n", ms, SIMD_N / (ms / 1000));
        System.out.printf("  [check: total=%.1f]%n", total);
    }

    // 5. Atomic
    static void benchAtomic() {
        AtomicInteger atomicVal = new AtomicInteger(0);
        for (int w = 0; w < WARMUP; w++) {
            for (int i = 0; i < N; i++) atomicVal.incrementAndGet();
        }

        long t0 = System.nanoTime();
        for (int i = 0; i < N; i++) atomicVal.incrementAndGet();
        long t1 = System.nanoTime();
        double msAtomic = (t1 - t0) / 1e6;
        System.out.printf("cpu_atomic_add: %.2f ms (%.0f ops/sec)%n", msAtomic, N / (msAtomic / 1000));

        for (int w = 0; w < WARMUP; w++) {
            int r = 0;
            for (int i = 0; i < N; i++) r++;
        }

        long t0r = System.nanoTime();
        int regularVal = 0;
        for (int i = 0; i < N; i++) regularVal++;
        long t1r = System.nanoTime();
        double msReg = (t1r - t0r) / 1e6;
        System.out.printf("cpu_nonatomic_add: %.2f ms (%.0f ops/sec, %.1fx faster)%n", msReg, N / (msReg / 1000), msAtomic / msReg);
    }
}
