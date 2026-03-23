import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import java.util.ArrayList;
import java.util.List;

public class ConcurrencyBench {
    static final int NUM_THREADS = 8;
    static final long OPS_PER_THREAD = 10_000_000;
    
    public static void main(String[] args) throws Exception {
        System.out.println("Java Concurrency Benchmark (" + NUM_THREADS + " threads, "
            + OPS_PER_THREAD + " ops/thread)");
        
        AtomicLong counter = new AtomicLong(0);
        long t0 = System.nanoTime();
        List<Thread> threads = new ArrayList<>();
        for (int t = 0; t < NUM_THREADS; t++) {
            threads.add(new Thread(() -> {
                for (long i = 0; i < OPS_PER_THREAD; i++) {
                    counter.incrementAndGet();
                }
            }));
        }
        for (Thread th : threads) th.start();
        for (Thread th : threads) th.join();
        long t1 = System.nanoTime();
        long totalOps = (long) NUM_THREADS * OPS_PER_THREAD;
        System.out.printf("Atomic: %.2f ms (%.0f ops/sec)%n",
            (t1 - t0) / 1e6, totalOps / ((t1 - t0) / 1e9));
        
        t0 = System.nanoTime();
        ExecutorService executor = Executors.newFixedThreadPool(NUM_THREADS);
        List<Future<Long>> futures = new ArrayList<>();
        for (int t = 0; t < NUM_THREADS; t++) {
            final long start = t * OPS_PER_THREAD;
            final long end = start + OPS_PER_THREAD;
            futures.add(executor.submit(() -> {
                long sum = 0;
                for (long i = start; i < end; i++) {
                    sum += i * i;
                }
                return sum;
            }));
        }
        long sum = 0;
        for (Future<Long> f : futures) sum += f.get();
        t1 = System.nanoTime();
        System.out.printf("Parallel sum: %.2f ms (%.0f ops/sec)%n",
            (t1 - t0) / 1e6, totalOps / ((t1 - t0) / 1e9));
        
        t0 = System.nanoTime();
        List<Future<Long>> poolFutures = new ArrayList<>();
        for (int t = 0; t < NUM_THREADS; t++) {
            final long start = t * OPS_PER_THREAD;
            poolFutures.add(executor.submit(() -> {
                long sum2 = 0;
                for (long i = start; i < start + OPS_PER_THREAD; i++) {
                    sum2 += i * i;
                }
                return sum2;
            }));
        }
        for (Future<Long> f : poolFutures) f.get();
        t1 = System.nanoTime();
        executor.shutdown();
        System.out.printf("Thread pool: %.2f ms (%.0f ops/sec)%n",
            (t1 - t0) / 1e6, totalOps / ((t1 - t0) / 1e9));
    }
}
