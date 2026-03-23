public class StringBench {
    static final int STR_LEN = 1000;
    static final int ITERATIONS = 100000;
    
    static int hashDjb2(String s) {
        int hash = 5381;
        for (int i = 0; i < s.length(); i++) {
            hash = ((hash << 5) + hash) + s.charAt(i);
        }
        return hash;
    }
    
    static long countSplits(String s, char delim) {
        long count = 0;
        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == delim) count++;
        }
        return count + 1;
    }
    
    public static void main(String[] args) {
        System.out.println("Java String Operations (" + ITERATIONS + " iterations)");
        
        StringBuilder sb = new StringBuilder(STR_LEN);
        for (int i = 0; i < STR_LEN; i++) {
            sb.append((char)('a' + (i % 26)));
        }
        String testStr = sb.toString();
        
        long t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            hashDjb2(testStr);
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("Hash: %.2f ms (%.0f ops/sec)%n", 
            ms, ITERATIONS / (ms / 1000));
        
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            String rev = new StringBuilder(testStr).reverse().toString();
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Reverse: %.2f ms (%.0f ops/sec)%n", 
            ms, ITERATIONS / (ms / 1000));
        
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            countSplits(testStr, ',');
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Split: %.2f ms (%.0f ops/sec)%n", 
            ms, ITERATIONS / (ms / 1000));
    }
}
