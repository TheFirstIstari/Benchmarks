#include <iostream>
#include <vector>
#include <string>

int fibonacci(int n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

double matrix_multiply(double a[3][3], double b[3][3]) {
    double c[3][3] = {0};
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            for (int k = 0; k < 3; k++) {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    double sum = 0;
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            sum += c[i][j];
        }
    }
    return sum;
}

int main() {
    volatile int result = 0;
    for (int i = 0; i < 30; i++) {
        result += fibonacci(i);
    }
    
    double a[3][3], b[3][3];
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            a[i][j] = i * 3.0 + j + 1.0;
            b[i][j] = j * 3.0 + i + 1.0;
        }
    }
    
    volatile double sum = 0;
    for (int i = 0; i < 1000000; i++) {
        sum += matrix_multiply(a, b);
    }
    
    return result + (int)sum;
}
