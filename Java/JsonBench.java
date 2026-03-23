import java.util.HashMap;
import java.util.Map;

public class JsonBench {
    static final int ITERATIONS = 5000;
    
    public static void main(String[] args) {
        System.out.println("Java JSON Benchmark (" + ITERATIONS + " iterations)");
        
        String testJson = "{\"name\":\"test\",\"age\":42,\"active\":true,\"scores\":[1,2,3,4,5,6,7,8,9,10]}";
        
        long t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            String[] parts = testJson.replace("{", "").replace("}", "").split(",");
        }
        long t1 = System.nanoTime();
        double ms = (t1 - t0) / 1e6;
        System.out.printf("Parse: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        StringBuilder sb = new StringBuilder();
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            sb.setLength(0);
            sb.append("{\"id\":").append(i)
              .append(",\"name\":\"user\"")
              .append(",\"value\":").append(i / 10.0)
              .append(",\"items\":[1,2,3,4,5,6,7,8,9,10]}");
            String result = sb.toString();
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Serialize: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        StringBuilder largeStr = new StringBuilder("{\"data\": \"");
        for (int i = 0; i < 1000000; i++) {
            largeStr.append("x");
        }
        largeStr.append("\"}");
        String large = largeStr.toString();
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            int pos = large.indexOf("{\"");
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Search: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
        
        t0 = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            int count = 0;
            int pos = 0;
            while ((pos = large.indexOf("\"\":", pos)) != -1) {
                count++;
                pos++;
            }
        }
        t1 = System.nanoTime();
        ms = (t1 - t0) / 1e6;
        System.out.printf("Count fields: %.2f ms (%.0f ops/sec)%n", ms, ITERATIONS / (ms / 1000));
    }
}
