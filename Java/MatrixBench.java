public class MatrixBench {
    static final int N = 2000;
    static final int ITERATIONS = 5;
    
    static void matmul(double[] C, double[] A, double[] B, int n) {
        for (int i = 0; i < n; i++) {
            for (int k = 0; k < n; k++) {
                double aik = A[i * n + k];
                for (int j = 0; j < n; j++) {
                    C[i * n + j] += aik * B[k * n + j];
                }
            }
        }
    }
    
    static void transpose(double[] T, double[] M, int n) {
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                T[j * n + i] = M[i * n + j];
            }
        }
    }
    
    static void add(double[] C, double[] A, double[] B, int n) {
        for (int i = 0; i < n * n; i++) {
            C[i] = A[i] + B[i];
        }
    }
    
    public static void main(String[] args) {
        System.out.println("Java Matrix Benchmark (" + N + "x" + N + ", " + ITERATIONS + " iterations)");
        
        double[] A = new double[N * N];
        double[] B = new double[N * N];
        double[] C = new double[N * N];
        double[] T = new double[N * N];
        
        java.util.Random rng = new java.util.Random(42);
        for (int i = 0; i < N * N; i++) {
            A[i] = rng.nextDouble();
            B[i] = rng.nextDouble();
        }
        
        long totalNs = 0;
        double checksum = 0;
        
        for (int iter = 0; iter < ITERATIONS; iter++) {
            for (int i = 0; i < N * N; i++) C[i] = 0;
            
            long t0 = System.nanoTime();
            matmul(C, A, B, N);
            long t1 = System.nanoTime();
            
            totalNs += t1 - t0;
            for (double v : C) checksum += v;
        }
        
        double avgMs = (totalNs / ITERATIONS) / 1_000_000.0;
        double ops = (double) N * N * N * 2;
        double gflops = (ops * ITERATIONS) / (totalNs / 1e9) / 1e9;
        
        System.out.printf("Multiply: %.2f ms (%.2f GFLOPS)%n", avgMs, gflops);
        
        long t0 = System.nanoTime();
        for (int iter = 0; iter < ITERATIONS; iter++) {
            transpose(T, A, N);
            for (double v : T) checksum += v;
        }
        long t1 = System.nanoTime();
        avgMs = (t1 - t0) / ITERATIONS / 1_000_000.0;
        System.out.printf("Transpose: %.2f ms%n", avgMs);
        
        t0 = System.nanoTime();
        for (int iter = 0; iter < ITERATIONS; iter++) {
            add(C, A, B, N);
        }
        t1 = System.nanoTime();
        avgMs = (t1 - t0) / ITERATIONS / 1_000_000.0;
        System.out.printf("Add: %.2f ms%n", avgMs);
    }
}
