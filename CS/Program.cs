using System;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Net.Sockets;
using System.Numerics;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;

class Program
{
    static int Main(string[] args)
    {
        string subcommand = args.Length > 0 ? args[0] : "all";

        if (subcommand == "all" || subcommand == "string") StringBench();
        if (subcommand == "all" || subcommand == "sort") SortBench();
        if (subcommand == "all" || subcommand == "matrix") MatrixBench();
        if (subcommand == "all" || subcommand == "hash") HashBench();
        if (subcommand == "all" || subcommand == "regex") RegexBench();
        if (subcommand == "all" || subcommand == "json") JsonBench();
        if (subcommand == "all" || subcommand == "fileio") FileIoBench();
        if (subcommand == "all" || subcommand == "math") MathBench();
        if (subcommand == "all" || subcommand == "network") NetworkBench();
        if (subcommand == "all" || subcommand == "crypto") CryptoBench();
        if (subcommand == "all" || subcommand == "ml") MlBench();
        if (subcommand == "all" || subcommand == "concurrency") ConcurrencyBench();
        if (subcommand == "all" || subcommand == "cpu") CpuBench();
        if (subcommand == "all" || subcommand == "allocator") AllocatorBench();

        return 0;
    }

    static void StringBench()
    {
        const int STR_LEN = 1000;
        const int ITERATIONS = 100000;
        Console.WriteLine($"C# String Operations ({ITERATIONS} iterations)");

        char[] chars = new char[STR_LEN];
        for (int i = 0; i < STR_LEN; i++) chars[i] = (char)('a' + (i % 26));
        string testStr = new string(chars);

        var sw = Stopwatch.StartNew();
        uint total = 0;
        for (int i = 0; i < ITERATIONS; i++) total += HashDjb2(testStr);
        sw.Stop();
        Console.WriteLine($"Hash: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            Span<char> arr = testStr.ToArray();
            arr.Reverse();
        }
        sw.Stop();
        Console.WriteLine($"Reverse: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        int parts = 0;
        for (int i = 0; i < ITERATIONS; i++) parts += CountSplits(testStr, ',');
        sw.Stop();
        Console.WriteLine($"Split: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static uint HashDjb2(ReadOnlySpan<char> s)
    {
        uint hash = 5381;
        foreach (char c in s) hash = ((hash << 5) + hash) + (byte)c;
        return hash;
    }

    static int CountSplits(ReadOnlySpan<char> s, char delim)
    {
        int count = 0;
        foreach (char c in s) if (c == delim) count++;
        return count + 1;
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
        Console.WriteLine($"Sort: {totalMs / ITER:F2} ms");
    }

    static void MatrixBench()
    {
        const int N = 2000;
        const int ITER = 5;
        Console.WriteLine($"C# Matrix Multiplication ({N}x{N}, {ITER} iterations)");

        double[] A = new double[N * N];
        double[] B = new double[N * N];
        double[] C = new double[N * N];
        double[] T = new double[N * N];
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
                for (int k = 0; k < N; k++)
                {
                    double aik = A[i * N + k];
                    for (int j = 0; j < N; j++)
                        C[i * N + j] += aik * B[k * N + j];
                }
            sw.Stop();
            totalMs += sw.Elapsed.TotalMilliseconds;
            for (int i = 0; i < N * N; i++) checksum += C[i];
        }

        double avgMs = totalMs / ITER;
        double ops = (double)N * N * N * 2;
        double gflops = (ops * ITER) / (totalMs / 1000.0) / 1e9;
        Console.WriteLine($"Multiply: {avgMs:F2} ms");
        Console.WriteLine($"Performance: {gflops:F2} GFLOPS");
        Console.WriteLine($"Checksum: {checksum:F2e}");

        var swT = Stopwatch.StartNew();
        for (int iter = 0; iter < ITER; iter++)
        {
            for (int i = 0; i < N; i++)
                for (int j = 0; j < N; j++)
                    T[j * N + i] = A[i * N + j];
            for (int i = 0; i < N * N; i++) checksum += T[i];
        }
        swT.Stop();
        Console.WriteLine($"Transpose: {swT.Elapsed.TotalMilliseconds / ITER:F2} ms");

        var swA = Stopwatch.StartNew();
        for (int iter = 0; iter < ITER; iter++)
            for (int i = 0; i < N * N; i++) C[i] = A[i] + B[i];
        swA.Stop();
        Console.WriteLine($"Add: {swA.Elapsed.TotalMilliseconds / ITER:F2} ms");
    }

    static void HashBench()
    {
        const int ITERATIONS = 1_000_000;
        const int STR_LEN = 1000;
        Console.WriteLine($"C# Hashing Benchmark ({ITERATIONS} iterations)");

        byte[] data = new byte[STR_LEN];
        for (int i = 0; i < STR_LEN; i++) data[i] = (byte)'a';
        uint result = 0;

        var sw = Stopwatch.StartNew();
        for (int i = 0; i < ITERATIONS; i++) result += HashSdbm(data);
        sw.Stop();
        Console.WriteLine($"SDBM: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++) result += HashDjb2B(data);
        sw.Stop();
        Console.WriteLine($"DJB2: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++) result += HashFnv(data);
        sw.Stop();
        Console.WriteLine($"FNV: {sw.Elapsed.TotalMilliseconds:F2} ms");

        if (result == 0) Console.WriteLine("");
    }

    static uint HashSdbm(byte[] s)
    {
        uint hash = 0;
        for (int i = 0; i < s.Length; i++) hash = (uint)(s[i] + (hash << 6) + (hash << 16) - hash);
        return hash;
    }

    static uint HashDjb2B(byte[] s)
    {
        uint hash = 5381;
        for (int i = 0; i < s.Length; i++) hash = ((hash << 5) + hash) + s[i];
        return hash;
    }

    static uint HashFnv(byte[] s)
    {
        uint hash = 2166136261u;
        for (int i = 0; i < s.Length; i++)
        {
            hash ^= s[i];
            hash *= 16777619u;
        }
        return hash;
    }

    static void RegexBench()
    {
        const int ITERATIONS = 1000;
        const int STR_LEN = 100000;
        Console.WriteLine($"C# Regex Benchmark ({ITERATIONS} iterations)");

        char[] c = new char[STR_LEN];
        for (int i = 0; i < STR_LEN; i++) c[i] = (char)('a' + (i % 26));
        string testStr = new string(c);

        var reFind = new Regex("xyz");
        var reCount = new Regex("[aeiou]");
        var reEmail = new Regex("[a-z]+@[a-z]+\\.[a-z]+");
        var reComplex = new Regex("[0-9]{3}-[0-9]{3}-[0-9]{4}");

        var sw = Stopwatch.StartNew();
        for (int i = 0; i < ITERATIONS; i++) reFind.Match(testStr);
        sw.Stop();
        Console.WriteLine($"Find: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            int count = 0;
            var m = reCount.Match(testStr);
            while (m.Success) { count++; m = m.NextMatch(); }
        }
        sw.Stop();
        Console.WriteLine($"Count: {sw.Elapsed.TotalMilliseconds:F2} ms");

        string emailStr = testStr.Substring(0, 100);
        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++) reEmail.Match(emailStr);
        sw.Stop();
        Console.WriteLine($"Email match: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++) reComplex.Match(testStr);
        sw.Stop();
        Console.WriteLine($"Complex pattern: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void JsonBench()
    {
        const int ITERATIONS = 5000;
        const int STR_LEN = 1_000_000;
        Console.WriteLine($"C# JSON Benchmark ({ITERATIONS} iterations)");

        char[] data = new char[STR_LEN];
        for (int i = 0; i < STR_LEN; i++) data[i] = 'a';
        string testStr = new string(data);

        var sw = Stopwatch.StartNew();
        for (int i = 0; i < ITERATIONS; i++)
        {
            char[] copy = new char[STR_LEN];
            testStr.CopyTo(0, copy, 0, STR_LEN);
        }
        sw.Stop();
        Console.WriteLine($"Parse: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            string output = $"{{\"id\":{i},\"name\":\"user\",\"value\":{(double)i / 10.0:F2},\"items\":[1,2,3,4,5,6,7,8,9,10]}}";
            GC.KeepAlive(output);
        }
        sw.Stop();
        Console.WriteLine($"Serialize: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            for (int j = 0; j < STR_LEN - 10; j++)
            {
                if (testStr[j] == '{' && testStr[j + 1] == '"') break;
            }
        }
        sw.Stop();
        Console.WriteLine($"Search: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            int count = 0;
            for (int j = 0; j < STR_LEN - 1; j++)
                if (testStr[j] == '"' && testStr[j + 1] == ':') count++;
        }
        sw.Stop();
        Console.WriteLine($"Count fields: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void FileIoBench()
    {
        const int ITERATIONS = 1000;
        const int FILE_SIZE = 5_000_000;
        Console.WriteLine($"C# File I/O Benchmark ({ITERATIONS} iterations)");

        byte[] data = new byte[FILE_SIZE];
        for (int i = 0; i < FILE_SIZE; i++) data[i] = (byte)('a' + (i % 26));
        string path = Path.Combine(Path.GetTempPath(), "bench_io_cs.dat");
        if (File.Exists(path)) File.Delete(path);

        var sw = Stopwatch.StartNew();
        for (int i = 0; i < ITERATIONS; i++) File.WriteAllBytes(path, data);
        sw.Stop();
        Console.WriteLine($"Write: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++) data = File.ReadAllBytes(path);
        sw.Stop();
        Console.WriteLine($"Read: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            int count = 0;
            using (var sr = new StreamReader(path))
            {
                while (sr.ReadLine() != null) count++;
            }
        }
        sw.Stop();
        Console.WriteLine($"Read lines: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS; i++)
        {
            using (var fs = new FileStream(path, FileMode.Open, FileAccess.Read))
            {
                fs.Seek(FILE_SIZE / 2, SeekOrigin.Begin);
                byte[] buf = new byte[1024];
                fs.ReadExactly(buf);
            }
        }
        sw.Stop();
        Console.WriteLine($"Random access: {sw.Elapsed.TotalMilliseconds:F2} ms");

        if (File.Exists(path)) File.Delete(path);
    }

    static void MathBench()
    {
        const int ITERATIONS = 10_000_000;
        Console.WriteLine($"C# Math Benchmark ({ITERATIONS} iterations)");

        double result = 0;
        var sw = Stopwatch.StartNew();
        for (int i = 1; i < ITERATIONS; i++)
            result += Math.Sqrt(i) + Math.Sin(i) + Math.Cos(i);
        sw.Stop();
        Console.WriteLine($"Trig functions: {sw.Elapsed.TotalMilliseconds:F2} ms");

        result = 0;
        sw.Restart();
        for (int i = 1; i < ITERATIONS; i++)
            result += Math.Log(i) + Math.Exp(i % 10);
        sw.Stop();
        Console.WriteLine($"Exp/log functions: {sw.Elapsed.TotalMilliseconds:F2} ms");

        result = 0;
        sw.Restart();
        for (int i = 1; i < ITERATIONS; i++)
        {
            double x = i;
            result += x * x + x * x * x + Math.Sqrt(x * x + 1.0);
        }
        sw.Stop();
        Console.WriteLine($"Arithmetic: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void NetworkBench()
    {
        const int ITERATIONS = 100;
        const int BUFFER_SIZE = 65536;
        Console.WriteLine($"C# Network Benchmark ({ITERATIONS} iterations)");

        var server = new TcpListener(IPAddress.Loopback, 0);
        server.Start();
        int port = ((IPEndPoint)server.LocalEndpoint).Port;

        byte[] clientBuf = new byte[BUFFER_SIZE];
        for (int i = 0; i < BUFFER_SIZE; i++) clientBuf[i] = (byte)'A';

        var clientTask = Task.Run(() =>
        {
            using var client = new TcpClient();
            client.Connect(IPAddress.Loopback, port);
            using var ns = client.GetStream();
            byte[] readBuf = new byte[BUFFER_SIZE];
            for (int i = 0; i < ITERATIONS; i++)
            {
                ns.Write(clientBuf, 0, BUFFER_SIZE);
                int read = 0;
                while (read < BUFFER_SIZE) read += ns.Read(readBuf, read, BUFFER_SIZE - read);
            }
        });

        using (var serverClient = server.AcceptTcpClient())
        using (var ns = serverClient.GetStream())
        {
            byte[] buf = new byte[BUFFER_SIZE];
            for (int i = 0; i < ITERATIONS; i++)
            {
                int read = 0;
                while (read < BUFFER_SIZE) read += ns.Read(buf, read, BUFFER_SIZE - read);
                ns.Write(buf, 0, BUFFER_SIZE);
            }
        }
        clientTask.Wait();
        server.Stop();

        var sw = Stopwatch.StartNew();
        for (int i = 0; i < ITERATIONS; i++)
        {
            Buffer.BlockCopy(clientBuf, 0, clientBuf, 4096, 4096);
        }
        sw.Stop();
        Console.WriteLine($"Memcpy 8K: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int i = 0; i < ITERATIONS * 100; i++)
        {
            Volatile.Write(ref clientBuf[0], (byte)(clientBuf[0] + 1));
        }
        sw.Stop();
        Console.WriteLine($"Loop overhead: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void CryptoBench()
    {
        const int ITERATIONS = 1000;
        const int BLOCK_SIZE = 16;
        Console.WriteLine($"C# Crypto Benchmark ({ITERATIONS} iterations)");

        byte[] key = new byte[BLOCK_SIZE];
        byte[] input = new byte[BLOCK_SIZE];
        byte[] output = new byte[BLOCK_SIZE];
        for (int i = 0; i < BLOCK_SIZE; i++) { key[i] = (byte)i; input[i] = 0xAA; }

        var sw = Stopwatch.StartNew();
        for (int i = 0; i < ITERATIONS; i++)
            for (int j = 0; j < BLOCK_SIZE; j++) output[j] = (byte)(input[j] ^ key[j]);
        sw.Stop();
        Console.WriteLine($"AES SIMD (xor): {sw.Elapsed.TotalMilliseconds:F2} ms");

        int checksum = 0;
        sw.Restart();
        for (int i = 0; i < ITERATIONS * 1000; i++) checksum += output[i % BLOCK_SIZE];
        sw.Stop();
        Console.WriteLine($"Integer ops: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void MlBench()
    {
        const int ITERATIONS = 100;
        const int VECTOR_SIZE = 1024;
        Console.WriteLine($"C# ML Benchmark ({ITERATIONS} iterations, size {VECTOR_SIZE})");

        float[] a = new float[VECTOR_SIZE];
        float[] b = new float[VECTOR_SIZE];
        float[] c = new float[VECTOR_SIZE];
        for (int i = 0; i < VECTOR_SIZE; i++)
        {
            a[i] = (float)i / VECTOR_SIZE;
            b[i] = (float)(VECTOR_SIZE - i) / VECTOR_SIZE;
            c[i] = 0;
        }

        var sw = Stopwatch.StartNew();
        for (int iter = 0; iter < ITERATIONS; iter++)
            for (int i = 0; i < VECTOR_SIZE; i++) c[i] = a[i] * b[i];
        sw.Stop();
        Console.WriteLine($"Element-wise mul: {sw.Elapsed.TotalMilliseconds:F2} ms");

        float dot = 0;
        sw.Restart();
        for (int iter = 0; iter < ITERATIONS; iter++)
        {
            dot = 0;
            for (int i = 0; i < VECTOR_SIZE; i++) dot += a[i] * b[i];
        }
        sw.Stop();
        Console.WriteLine($"Dot product: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int iter = 0; iter < ITERATIONS; iter++)
            for (int i = 0; i < VECTOR_SIZE; i++) a[i] = a[i] * 0.9f + b[i] * 0.1f;
        sw.Stop();
        Console.WriteLine($"Lerp (mix): {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int iter = 0; iter < ITERATIONS; iter++)
            for (int i = 0; i < VECTOR_SIZE; i++) c[i] = 1.0f / (1.0f + a[i]);
        sw.Stop();
        Console.WriteLine($"Sigmoid: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void ConcurrencyBench()
    {
        const int NUM_THREADS = 8;
        const int OPS_PER_THREAD = 10_000_000;
        Console.WriteLine($"C# Concurrency Benchmark ({NUM_THREADS} threads, {OPS_PER_THREAD} ops/thread)");

        long counter = 0;
        var sw = Stopwatch.StartNew();
        Task[] tasks = new Task[NUM_THREADS];
        for (int t = 0; t < NUM_THREADS; t++)
            tasks[t] = Task.Run(() =>
            {
                for (long i = 0; i < OPS_PER_THREAD; i++) Interlocked.Increment(ref counter);
            });
        Task.WaitAll(tasks);
        sw.Stop();
        Console.WriteLine($"Atomic: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int t = 0; t < NUM_THREADS; t++)
        {
            long tid = t;
            tasks[t] = Task.Run(() =>
            {
                long start = tid * OPS_PER_THREAD;
                long local = 0;
                for (long i = start; i < start + OPS_PER_THREAD; i++) local += i * i;
                return (object)local;
            });
        }
        Task.WaitAll(tasks);
        sw.Stop();
        Console.WriteLine($"Parallel sum: {sw.Elapsed.TotalMilliseconds:F2} ms");

        sw.Restart();
        for (int t = 0; t < NUM_THREADS; t++)
        {
            long tid = t;
            tasks[t] = Task.Run(() =>
            {
                long start = tid * OPS_PER_THREAD;
                long local = 0;
                for (long i = start; i < start + OPS_PER_THREAD; i++) local += i * i;
                return (object)local;
            });
        }
        Task.WaitAll(tasks);
        sw.Stop();
        Console.WriteLine($"Thread pool: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void CpuBench()
    {
        const int N = 10_000_000;
        const int BRANCH_N = 100_000_000;
        const int POINTER_N = 50_000_000;
        const int SIMD_N = 50_000_000;
        Console.WriteLine("C# CPU Benchmark");
        Console.WriteLine("--- Integer Arithmetic ---");

        ulong fibResult = 0;
        var sw = Stopwatch.StartNew();
        for (int j = 0; j < N; j++)
        {
            ulong fa = 0, fb = 1;
            for (int i = 0; i < 50; i++) { ulong fc = fa + fb; fa = fb; fb = fc; }
            fibResult += fa;
        }
        sw.Stop();
        Console.WriteLine($"cpu_fibonacci: {sw.Elapsed.TotalMilliseconds:F2} ms");

        int primeLimit = 10_000_000;
        sw.Restart();
        bool[] sieve = new bool[primeLimit + 1];
        int primeCount = 0;
        for (int i = 2; i <= primeLimit; i++)
        {
            if (!sieve[i])
            {
                primeCount++;
                if ((long)i * i <= primeLimit)
                    for (int j = i * i; j <= primeLimit; j += i) sieve[j] = true;
            }
        }
        sw.Stop();
        Console.WriteLine($"cpu_prime_sieve: {sw.Elapsed.TotalMilliseconds:F2} ms");

        int[] pcData = new int[N];
        var rand = new Random(42);
        for (int i = 0; i < N; i++) pcData[i] = rand.Next();
        long pcTotal = 0;
        sw.Restart();
        for (int i = 0; i < N; i++) pcTotal += BitOperations.PopCount((uint)pcData[i]);
        sw.Stop();
        Console.WriteLine($"cpu_popcount: {sw.Elapsed.TotalMilliseconds:F2} ms");

        Console.WriteLine("\n--- Branch Prediction ---");
        int[] sorted = new int[BRANCH_N];
        int[] unsorted = new int[BRANCH_N];
        for (int i = 0; i < BRANCH_N; i++) { sorted[i] = i; unsorted[i] = rand.Next() % 100; }
        long sumSorted = 0;
        sw.Restart();
        for (int i = 0; i < BRANCH_N; i++) if (sorted[i] > 50000000) sumSorted += sorted[i];
        sw.Stop();
        double msSorted = sw.Elapsed.TotalMilliseconds;
        Console.WriteLine($"cpu_branch_sorted: {msSorted:F2} ms");

        long sumUnsorted = 0;
        sw.Restart();
        for (int i = 0; i < BRANCH_N; i++) if (unsorted[i] > 50) sumUnsorted += unsorted[i];
        sw.Stop();
        double msUnsorted = sw.Elapsed.TotalMilliseconds;
        Console.WriteLine($"cpu_branch_unsorted: {msUnsorted:F2} ms");

        Console.WriteLine("\n--- Memory Latency ---");
        int[] indices = new int[POINTER_N];
        for (int i = 0; i < POINTER_N; i++) indices[i] = (i + 1) % POINTER_N;
        for (int i = POINTER_N - 1; i > 0; i--)
        {
            int j = rand.Next(i + 1);
            int tmp = indices[i]; indices[i] = indices[j]; indices[j] = tmp;
        }
        int pos = 0;
        sw.Restart();
        for (int i = 0; i < POINTER_N; i++) pos = indices[pos];
        sw.Stop();
        Console.WriteLine($"cpu_memory_latency: {sw.Elapsed.TotalMilliseconds:F2} ms");

        Console.WriteLine("\n--- SIMD Vectorization ---");
        float[] sa = new float[SIMD_N];
        float[] sb = new float[SIMD_N];
        float[] sc = new float[SIMD_N];
        for (int i = 0; i < SIMD_N; i++) { sa[i] = i * 1.0f; sb[i] = (i % 100) * 1.0f; }
        float simdResult = 0;
        sw.Restart();
        for (int i = 0; i < SIMD_N; i++) sc[i] = (sa[i] + sb[i]) * sa[i];
        sw.Stop();
        for (int i = 0; i < 1000; i++) simdResult += sc[i * 1000];
        Console.WriteLine($"cpu_simd_scalar: {sw.Elapsed.TotalMilliseconds:F2} ms");

        Console.WriteLine("\n--- Atomic Operations ---");
        int atomicVal = 0;
        sw.Restart();
        for (int i = 0; i < N; i++) Interlocked.Increment(ref atomicVal);
        sw.Stop();
        double msAtomic = sw.Elapsed.TotalMilliseconds;
        Console.WriteLine($"cpu_atomic_add: {msAtomic:F2} ms");

        int regularVal = 0;
        sw.Restart();
        for (int i = 0; i < N; i++) regularVal++;
        sw.Stop();
        Console.WriteLine($"cpu_nonatomic_add: {sw.Elapsed.TotalMilliseconds:F2} ms");
    }

    static void AllocatorBench()
    {
        const int BLOCK_SIZE = 64;
        const int LIVE_BLOCKS = 1024;
        const int TOTAL_ALLOCS = 10_000_000;
        const int NUM_THREADS = 4;
        Console.WriteLine($"C# Memory Allocator ({NUM_THREADS} threads, {LIVE_BLOCKS} blocks, {TOTAL_ALLOCS} total allocs)");

        var sw = Stopwatch.StartNew();
        Task[] tasks = new Task[NUM_THREADS];
        for (int t = 0; t < NUM_THREADS; t++)
        {
            tasks[t] = Task.Run(() =>
            {
                int perThread = TOTAL_ALLOCS / NUM_THREADS;
                byte[][] blocks = new byte[LIVE_BLOCKS][];
                int top = 0;
                for (int i = 0; i < perThread; i++)
                {
                    blocks[top++] = new byte[BLOCK_SIZE];
                    if (top >= LIVE_BLOCKS)
                    {
                        for (int j = 0; j < LIVE_BLOCKS / 2; j++) blocks[j] = null;
                        Array.Copy(blocks, LIVE_BLOCKS / 2, blocks, 0, LIVE_BLOCKS / 2);
                        top -= LIVE_BLOCKS / 2;
                    }
                }
                for (int i = 0; i < top; i++) blocks[i] = null;
            });
        }
        Task.WaitAll(tasks);
        sw.Stop();

        double ms = sw.Elapsed.TotalMilliseconds;
        Console.WriteLine($"Total time: {ms:F2} ms");
        Console.WriteLine($"Allocations/sec: {TOTAL_ALLOCS / (ms / 1000.0):F0}");
    }
}
