import java.io.*;
import java.nio.file.*;

public class FileIOBench {
    static final int ITERATIONS = 1000;
    static final int FILE_SIZE = 5000000;
    
    public static void main(String[] args) throws Exception {
        System.out.println("Java File I/O Benchmark (" + ITERATIONS + " iterations)");
        
        byte[] data = new byte[FILE_SIZE];
        for (int i = 0; i < FILE_SIZE; i++) {
            data[i] = (byte)('a' + (i % 26));
        }
        
        Files.deleteIfExists(Paths.get("/tmp/bench_io.dat"));
        
        long t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            try (FileOutputStream fos = new FileOutputStream("/tmp/bench_io.dat")) {
                fos.write(data);
            }
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("Write: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        byte[] readBuffer = new byte[FILE_SIZE];
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            try (FileInputStream fis = new FileInputStream("/tmp/bench_io.dat")) {
                fis.read(readBuffer);
            }
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Read: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            try (BufferedReader br = new BufferedReader(new FileReader("/tmp/bench_io.dat"))) {
                String line;
                while ((line = br.readLine()) != null) {}
            }
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Read lines: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        t0 = System.nanoTime();
        RandomAccessFile raf = new RandomAccessFile("/tmp/bench_io.dat", "r");
        for (int i = 0; i < ITERATIONS; i++) {
            raf.seek(FILE_SIZE / 2);
            raf.read(readBuffer, 0, 1024);
        }
        raf.close();
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Random access: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        Files.deleteIfExists(Paths.get("/tmp/bench_io.dat"));
    }
}
