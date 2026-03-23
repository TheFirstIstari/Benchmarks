public class HashBench {
    static final int ITERATIONS = 1000000;
    static final int STR_LEN = 1000;
    
    static int hashSdbm(String s) {
        int hash = 0;
        for (int i = 0; i < s.length(); i++) {
            hash = s.charAt(i) + (hash << 6) + (hash << 16) - hash;
        }
        return hash;
    }
    
    static int hashDjb2(String s) {
        int hash = 5381;
        for (int i = 0; i < s.length(); i++) {
            hash = ((hash << 5) + hash) + s.charAt(i);
        }
        return hash;
    }
    
    static int hashFnv(String s) {
        int hash = 0x811C9DC5;
        for (int i = 0; i < s.length(); i++) {
            hash ^= s.charAt(i);
            hash = (hash * 0x01000193) & 0xFFFFFFFF;
        }
        return hash;
    }
    
    public static void main(String[] args) {
        System.out.println("Java Hashing Benchmark (" + ITERATIONS + " iterations)");
        
        StringBuilder sb = new StringBuilder(STR_LEN);
        for (int i = 0; i < STR_LEN; i++) sb.append('a');
	        String data = sb.toString();
	        int result = 0;
        
        long t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            result += hashSdbm(data);
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("SDBM: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            result += hashDjb2(data);
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("DJB2: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            result += hashFnv(data);
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("FNV: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
    }
}
