import java.util.ArrayList;
import java.util.List;

public class CompileTest {
    static int fibonacci(int n) {
        if (n <= 1) return n;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
    
    static double matrixMultiply(double[][] a, double[][] b) {
        int n = a.length;
        double[][] c = new double[n][n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                for (int k = 0; k < n; k++) {
                    c[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        double sum = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                sum += c[i][j];
            }
        }
        return sum;
    }
    
    public static void main(String[] args) {
        int result = 0;
        for (int i = 0; i < 30; i++) {
            result += fibonacci(i);
        }
        
        int n = 3;
        double[][] a = new double[n][n];
        double[][] b = new double[n][n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                a[i][j] = i * 3.0 + j + 1.0;
                b[i][j] = j * 3.0 + i + 1.0;
            }
        }
        
        double sum = 0;
        for (int i = 0; i < 1000000; i++) {
            sum += matrixMultiply(a, b);
        }
        
        System.out.println("Result: " + (result + (int)sum));
    }
}
