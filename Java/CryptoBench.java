public class CryptoBench {
    static final int ITERATIONS = 1000;
    static final int BLOCK_SIZE = 16;
    
    public static void main(String[] args) {
        System.out.println("Java Crypto Benchmark");
        
        byte[] key = new byte[16];
        byte[] input = new byte[16];
        byte[] output = new byte[16];
        
        for (int i = 0; i < 16; i++) {
            key[i] = (byte) i;
            input[i] = (byte) 0xAA;
        }
        
        long t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            for (int j = 0; j < BLOCK_SIZE; j++) {
                output[j] = (byte) (input[j] ^ key[j]);
            }
        }
        long t1 = System.nanoTime();
        double totalMs = (t1 - t0) / 1e6;
        
        System.out.printf("XOR (simulated AES): %.2f ms%n", totalMs);
        
        int checksum = 0;
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS * 1000; i++) {
            checksum += output[i % 16];
        }
        t1 = System.nanoTime();
        totalMs = (t1 - t0) / 1e6;
        System.out.printf("Integer ops: %.2f ms (checksum: %d)%n", totalMs, checksum);
    }
}
