using System;
using System.Diagnostics;

class Program
{
    const int STR_LEN = 10000;
    const int ITERATIONS = 100000;
    
    static int HashDjb2(ReadOnlySpan<char> s)
    {
        int hash = 5381;
        foreach (char c in s) hash = ((hash << 5) + hash) + c;
        return hash;
    }
    
    static int CountSplits(ReadOnlySpan<char> s, char delim)
    {
        int count = 0;
        foreach (char c in s) if (c == delim) count++;
        return count + 1;
    }
    
    static void Main(string[] args)
    {
        string subcommand = args.Length > 0 ? args[0] : "all";
        
        if (subcommand == "all" || subcommand == "string")
        {
            StringBench();
        }
        if (subcommand == "all" || subcommand == "sort")
        {
            SortBench();
        }
        if (subcommand == "all" || subcommand == "matrix")
        {
            MatrixBench();
        }
    }
    
    static void StringBench()
    {
        Console.WriteLine($"C# String Operations ({ITERATIONS} iterations)");
        
        char[] chars = new char[STR_LEN];
        for (int i = 0; i < STR_LEN; i++)
        {
            chars[i] = (char)('a' + (i % 26));
        }
        string testStr = new string(chars);
        
        var sw = Stopwatch.StartNew();
        int total = 0;
        for (int i = 0; i < ITERATIONS; i++) total += HashDjb2(testStr);
        sw.Stop();
        Console.WriteLine($"Hash: {sw.Elapsed.TotalMilliseconds:F2} ms ({ITERATIONS / (sw.Elapsed.TotalMilliseconds / 1000):F0} ops/sec)");
        
        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            Span<char> arr = testStr.ToArray();
            arr.Reverse();
        }
        sw.Stop();
        Console.WriteLine($"Reverse: {sw.Elapsed.TotalMilliseconds:F2} ms ({ITERATIONS / (sw.Elapsed.TotalMilliseconds / 1000):F0} ops/sec)");
        
        sw.Restart();
        int parts = 0;
        for (int i = 0; i < ITERATIONS; i++) parts += CountSplits(testStr, ',');
        sw.Stop();
        Console.WriteLine($"Split: {sw.Elapsed.TotalMilliseconds:F2} ms ({ITERATIONS / (sw.Elapsed.TotalMilliseconds / 1000):F0} ops/sec)");
    }
    
    static void SortBench()
    {
        const int N = 5_000_000;
        const int ITER = 5;
        Console.WriteLine($"C# Sorting (N={N}, {ITER} iterations)");
        
        int[] original = new int[N];
        Random rng = new Random(42);
        for (int i = 0; i < N; i++) original[i] = rng.Next(1_000_000);
        
        int[] arr = new int[N];
        double totalMs = 0;
        
        for (int iter = 0; iter < ITER; iter++)
        {
            Array.Copy(original, arr, N);
            var sw = Stopwatch.StartNew();
            Array.Sort(arr);
            sw.Stop();
            totalMs += sw.Elapsed.TotalMilliseconds;
        }
        Console.WriteLine($"Sort: {totalMs / ITER:F2} ms avg ({N / (totalMs / ITER / 1000):F0} elements/sec)");
    }
    
    static void MatrixBench()
    {
        const int N = 1000;
        const int ITER = 10;
        Console.WriteLine($"C# Matrix Multiplication ({N}x{N}, {ITER} iterations)");
        
        double[] A = new double[N * N];
        double[] B = new double[N * N];
        double[] C = new double[N * N];
        Random rng = new Random(42);
        for (int i = 0; i < N * N; i++)
        {
            A[i] = rng.NextDouble();
            B[i] = rng.NextDouble();
        }
        
        double totalMs = 0;
        double checksum = 0;
        
        for (int iter = 0; iter < ITER; iter++)
        {
            Array.Clear(C, 0, C.Length);
            
            var sw = Stopwatch.StartNew();
            for (int i = 0; i < N; i++)
            {
                for (int k = 0; k < N; k++)
                {
                    double aik = A[i * N + k];
                    for (int j = 0; j < N; j++)
                    {
                        C[i * N + j] += aik * B[k * N + j];
                    }
                }
            }
            sw.Stop();
            
            totalMs += sw.Elapsed.TotalMilliseconds;
            for (int i = 0; i < N * N; i++) checksum += C[i];
        }
        
        double avgMs = totalMs / ITER;
        double ops = (double)N * N * N * 2;
        double gflops = (ops * ITER) / (totalMs / 1000.0) / 1e9;
        Console.WriteLine($"Average time: {avgMs:F2} ms");
        Console.WriteLine($"Performance: {gflops:F2} GFLOPS");
        Console.WriteLine($"Checksum: {checksum:F2e}");
    }
}
