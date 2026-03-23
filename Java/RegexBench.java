import java.util.regex.*;

public class RegexBench {
    static final int ITERATIONS = 1000;
    static final int STR_LEN = 100000;
    
    public static void main(String[] args) {
        System.out.println("Java Regex Benchmark (" + ITERATIONS + " iterations)");
        
        StringBuilder sb = new StringBuilder(STR_LEN);
        for (int i = 0; i < STR_LEN; i++) {
            sb.append((char)('a' + (i % 26)));
        }
        String testStr = sb.toString();
        
        long t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            Pattern.matches("xyz", testStr);
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("Find: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        Pattern countPattern = Pattern.compile("[aeiou]");
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            Matcher matcher = countPattern.matcher(testStr);
            int count = 0;
            while (matcher.find()) { count++; }
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Count: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        Pattern emailPattern = Pattern.compile("[a-z]+@[a-z]+\\.[a-z]+");
        String emailStr = testStr.substring(0, 100);
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            emailPattern.matcher(emailStr).find();
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Email match: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        Pattern complexPattern = Pattern.compile("[0-9]{3}-[0-9]{3}-[0-9]{4}");
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            complexPattern.matcher(testStr).find();
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Complex pattern: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
    }
}
