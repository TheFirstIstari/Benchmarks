import java.io.*;

public class NetworkBench {
    static final int ITERATIONS = 100;
    static final int BUFFER_SIZE = 65536;
    
    public static void main(String[] args) throws Exception {
        System.out.println("Java Network Benchmark");
        
        PipedInputStream in = new PipedInputStream();
        PipedOutputStream out = new PipedOutputStream(in);
        
        byte[] buffer = new byte[BUFFER_SIZE];
        byte[] readBuf = new byte[BUFFER_SIZE];
        
        long t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            out.write(buffer);
            in.read(readBuf);
        }
        long t1 = System.nanoTime();
        double totalMs = (t1 - t0) / 1e6;
        
        double throughput = (ITERATIONS * BUFFER_SIZE * 2) / (totalMs / 1000) / 1e9;
        System.out.printf("Pipe Throughput: %.2f GB/s (%.2f ms)%n", throughput, totalMs);
        
        byte[] src = new byte[8192];
        byte[] dst = new byte[8192];
        
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS * 10; i++) {
            System.arraycopy(src, 0, dst, 0, 8192);
        }
        t1 = System.nanoTime();
        totalMs = (t1 - t0) / 1e6;
        
        throughput = (ITERATIONS * 10 * 8192) / (totalMs / 1000) / 1e9;
        System.out.printf("Memcpy 8K: %.2f GB/s (%.2f ms)%n", throughput, totalMs);
    }
}
