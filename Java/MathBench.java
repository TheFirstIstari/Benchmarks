public class MathBench {
    static final int ITERATIONS = 10000000;
    static final int WARMUP = 3;
    
    public static void main(String[] args) {
        System.out.println("Java Math Benchmark (" + ITERATIONS + " iterations, " + WARMUP + " warmup runs)");
        
        for (int w = 0; w < WARMUP; w++) {
            double warmup = 0;
            for (int i = 1; i < ITERATIONS; i++) {
                warmup += Math.sqrt(i) + Math.sin(i) + Math.cos(i);
            }
        }
        
        long t0 = System.nanoTime();
        double result = 0;
        for (int i = 1; i < ITERATIONS; i++) {
            result += Math.sqrt(i) + Math.sin(i) + Math.cos(i);
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("Trig functions: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        for (int w = 0; w < WARMUP; w++) {
            double warmup = 0;
            for (int i = 1; i < ITERATIONS; i++) {
                warmup += Math.log(i) + Math.exp(i % 10);
            }
        }
        
        t0 = System.nanoTime();
        for (int i = 1; i < ITERATIONS; i++) {
            result += Math.log(i) + Math.exp(i % 10);
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Exp/log functions: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        for (int w = 0; w < WARMUP; w++) {
            double warmup = 0;
            for (int i = 1; i < ITERATIONS; i++) {
                double x = i;
                warmup += x * x + x * x * x + Math.sqrt(x * x + 1.0);
            }
        }
        
        t0 = System.nanoTime();
        for (int i = 1; i < ITERATIONS; i++) {
            double x = i;
            result += x * x + x * x * x + Math.sqrt(x * x + 1.0);
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Arithmetic: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        if (result == 0) System.out.println("");
    }
}
