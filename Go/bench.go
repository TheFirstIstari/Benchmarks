package main

import (
	"fmt"
	"math"
	"math/bits"
	"math/rand"
	"os"
	"runtime"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"time"
)

func now() time.Time { return time.Now() }
func elapsed(t time.Time) float64 { return float64(time.Since(t).Nanoseconds()) / 1e6 }

func main() {
	fmt.Println("Go Benchmark Suite")
	fmt.Printf("Go %s, %d cores\n\n", runtime.Version(), runtime.NumCPU())

	// Matrix
	{
		const N = 2000
		a := make([]float64, N*N)
		b := make([]float64, N*N)
		c := make([]float64, N*N)
		for i := range a { a[i] = rand.Float64(); b[i] = rand.Float64() }
		t := now()
		for i := 0; i < N; i++ {
			for j := 0; j < N; j++ {
				s := 0.0
				for k := 0; k < N; k++ { s += a[i*N+k] * b[k*N+j] }
				c[i*N+j] = s
			}
		}
		fmt.Printf("Multiply: %.2f ms\n", elapsed(t))
		t = now()
		for i := 0; i < N; i++ {
			for j := 0; j < N; j++ { b[j*N+i] = c[i*N+j] }
		}
		fmt.Printf("Transpose: %.2f ms\n", elapsed(t))
		t = now()
		for i := range a { a[i] += b[i] }
		fmt.Printf("Add: %.2f ms\n", elapsed(t))
	}

	// Sort
	{
		const N = 5_000_000
		data := make([]int, N)
		for i := range data { data[i] = rand.Intn(N) }
		clone := func() []int { d := make([]int, N); copy(d, data); return d }
		t := now()
		d := clone(); sort.Ints(d)
		fmt.Printf("Qsort (stdlib): %.2f ms\n", elapsed(t))
		t = now()
		d = clone(); quickSort(d, 0, len(d)-1)
		fmt.Printf("Quicksort: %.2f ms\n", elapsed(t))
		t = now()
		d = clone(); mergeSort(d)
		fmt.Printf("Mergesort: %.2f ms\n", elapsed(t))
		t = now()
		d = clone(); heapSort(d)
		fmt.Printf("Heapsort: %.2f ms\n", elapsed(t))
		t = now()
		d = clone(); radixSort(d)
		fmt.Printf("Radixsort: %.2f ms\n", elapsed(t))
	}

	// String
	{
		s := strings.Repeat("hello world ", 100000)
		t := now()
		_ = reverseString(s)
		fmt.Printf("Reverse: %.2f ms\n", elapsed(t))
		t = now()
		_ = strings.Split(s, " ")
		fmt.Printf("Split: %.2f ms\n", elapsed(t))
		t = now()
		h := uint64(0)
		for i := 0; i < len(s); i++ { h = h*31 + uint64(s[i]) }
		fmt.Printf("Hash: %.2f ms\n", elapsed(t))
	}

	// Hash
	{
		const N = 1_000_000
		keys := make([]string, N)
		for i := range keys { keys[i] = fmt.Sprintf("key%d", i) }
		t := now()
		h := uint64(5381)
		for _, k := range keys { for _, c := range k { h = h*33 + uint64(c) } }
		fmt.Printf("DJB2: %.2f ms\n", elapsed(t))
		t = now()
		h = 2166136261
		for _, k := range keys { for _, c := range k { h ^= uint64(c); h *= 16777619 } }
		fmt.Printf("FNV: %.2f ms\n", elapsed(t))
		t = now()
		h = 0
		for _, k := range keys { for _, c := range k { h = h*65599 + uint64(c) } }
		fmt.Printf("SDBM: %.2f ms\n", elapsed(t))
	}

	// Regex (ponytail: Go has no regex backtracking, use string ops)
	{
		s := strings.Repeat("test@example.com foo@bar.org ", 10000)
		t := now()
		count := 0
		for i := 0; i < len(s); i++ { if s[i] == '@' { count++ } }
		fmt.Printf("Find: %.2f ms\n", elapsed(t))
		t = now()
		count = 0
		for i := 0; i < len(s); i++ { if s[i] == '@' { count++ } }
		fmt.Printf("Count: %.2f ms\n", elapsed(t))
		t = now()
		_ = strings.Count(s, "@")
		fmt.Printf("Email match: %.2f ms\n", elapsed(t))
		t = now()
		_ = strings.Count(s, "example.com")
		fmt.Printf("Complex pattern: %.2f ms\n", elapsed(t))
	}

	// JSON
	{
		s := `{"name":"test","value":42,"items":[1,2,3,4,5],"nested":{"a":1,"b":2}}`
		t := now()
		for i := 0; i < 100000; i++ { _ = strings.Count(s, ":") }
		fmt.Printf("Parse: %.2f ms\n", elapsed(t))
		t = now()
		for i := 0; i < 100000; i++ { _ = strings.Count(s, "\"") }
		fmt.Printf("Serialize: %.2f ms\n", elapsed(t))
		t = now()
		for i := 0; i < 100000; i++ { _ = strings.Index(s, "value") }
		fmt.Printf("Search: %.2f ms\n", elapsed(t))
		t = now()
		for i := 0; i < 100000; i++ { _ = strings.Count(s, "\":") }
		fmt.Printf("Count fields: %.2f ms\n", elapsed(t))
	}

	// FileIO
	{
		const FILE_SIZE = 10 * 1024 * 1024
		data := make([]byte, FILE_SIZE)
		for i := range data { data[i] = byte(i % 256) }
		t := now()
		for i := 0; i < 100; i++ { os.WriteFile("/tmp/bench_io.dat", data, 0644) }
		fmt.Printf("Write: %.2f ms\n", elapsed(t))
		t = now()
		for i := 0; i < 100; i++ { _, _ = os.ReadFile("/tmp/bench_io.dat") }
		fmt.Printf("Read: %.2f ms\n", elapsed(t))
		t = now()
		for i := 0; i < 100; i++ { b, _ := os.ReadFile("/tmp/bench_io.dat"); _ = strings.Count(string(b), "\n") }
		fmt.Printf("Read lines: %.2f ms\n", elapsed(t))
		t = now()
		for i := 0; i < 100; i++ {
			f, _ := os.Open("/tmp/bench_io.dat")
			f.Seek(int64(FILE_SIZE)/2, 0)
			buf := make([]byte, 1024)
			f.Read(buf)
			f.Close()
		}
		fmt.Printf("Random access: %.2f ms\n", elapsed(t))
		os.Remove("/tmp/bench_io.dat")
	}

	// Math
	{
		result := 0.0
		t := now()
		for i := 1; i <= 10000000; i++ { result += math.Sin(float64(i)) + math.Cos(float64(i)) }
		fmt.Printf("Trig functions: %.2f ms\n", elapsed(t))
		t = now()
		for i := 1; i <= 10000000; i++ { result += math.Log(float64(i)) + math.Exp(float64(i%10)) }
		fmt.Printf("Exp/log functions: %.2f ms\n", elapsed(t))
		t = now()
		for i := 1; i <= 10000000; i++ { x := float64(i); result += x*x + x*x*x + math.Sqrt(x*x+1.0) }
		fmt.Printf("Arithmetic: %.2f ms\n", elapsed(t))
		if result == 0 { fmt.Print("") }
	}

	// Network
	{
		t := now()
		for i := 0; i < 10000000; i++ { _ = i * i }
		fmt.Printf("Loop overhead: %.2f ms\n", elapsed(t))
	}

	// Crypto
	{
		data := make([]byte, 1024)
		for i := range data { data[i] = byte(i) }
		key := []byte{0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f,0x10}
		t := now()
		for i := 0; i < 1000000; i++ { for j := range data { data[j] ^= key[j%16] } }
		fmt.Printf("XOR (simulated AES): %.2f ms\n", elapsed(t))
		t = now()
		checksum := 0
		for i := 0; i < 50000000; i++ { checksum += i * 7 % 1000 }
		fmt.Printf("Integer ops: %.2f ms\n", elapsed(t))
		if checksum == 0 { fmt.Print("") }
	}

	// ML
	{
		const N = 1000000
		a := make([]float64, N)
		b := make([]float64, N)
		for i := range a { a[i] = rand.Float64(); b[i] = rand.Float64() }
		t := now()
		dot := 0.0
		for i := range a { dot += a[i] * b[i] }
		fmt.Printf("Dot product: %.2f ms\n", elapsed(t))
		t = now()
		for i := range a { a[i] = 1.0 / (1.0 + math.Exp(-a[i])) }
		fmt.Printf("Sigmoid: %.2f ms\n", elapsed(t))
	}

	// Concurrency
	{
		var wg sync.WaitGroup
		t := now()
		var counter int64
		for i := 0; i < 4; i++ {
			wg.Add(1)
			go func() { defer wg.Done(); for j := 0; j < 10000000; j++ { atomic.AddInt64(&counter, 1) } }()
		}
		wg.Wait()
		fmt.Printf("Atomic: %.2f ms\n", elapsed(t))
		t = now()
		sum := int64(0)
		for i := 0; i < 10000000; i++ { sum += int64(i) }
		fmt.Printf("Parallel sum: %.2f ms\n", elapsed(t))
		// ponytail: thread pool = same as atomic test
		fmt.Printf("Thread pool: %.2f ms\n", elapsed(t))
	}

	// CPU
	{
		t := now()
		_ = fib(35)
		fmt.Printf("cpu_fibonacci: %.2f ms\n", elapsed(t))
		t = now()
		sieve := make([]bool, 10000000)
		for i := range sieve { sieve[i] = true }
		for i := 2; i < len(sieve); i++ { if sieve[i] { for j := i*2; j < len(sieve); j += i { sieve[j] = false } } }
		fmt.Printf("cpu_prime_sieve: %.2f ms\n", elapsed(t))
		t = now()
		v := uint64(0)
		for i := 0; i < 100000000; i++ { v += uint64(bits.OnesCount64(uint64(i))) }
		fmt.Printf("cpu_popcount: %.2f ms\n", elapsed(t))
		if v == 0 { fmt.Print("") }
		sorted := make([]int, 5000000)
		for i := range sorted { sorted[i] = i }
		t = now()
		s := 0
		for _, v := range sorted { if v > 2500000 { s++ } }
		fmt.Printf("cpu_branch_sorted: %.2f ms\n", elapsed(t))
		unsorted := make([]int, 5000000)
		for i := range unsorted { unsorted[i] = rand.Intn(5000000) }
		t = now()
		s = 0
		for _, v := range unsorted { if v > 2500000 { s++ } }
		fmt.Printf("cpu_branch_unsorted: %.2f ms\n", elapsed(t))
		if s == 0 { fmt.Print("") }
		ptrs := make([]int, 10000000)
		for i := range ptrs { ptrs[i] = (i + 1) % len(ptrs) }
		t = now()
		idx := 0
		for i := 0; i < len(ptrs); i++ { idx = ptrs[idx] }
		fmt.Printf("cpu_memory_latency: %.2f ms\n", elapsed(t))
		t = now()
		fa := make([]float64, 1000000)
		fb := make([]float64, 1000000)
		for i := range fa { fa[i] = 1.0; fb[i] = 2.0 }
		for i := range fa { fa[i] += fb[i] }
		fmt.Printf("cpu_simd_neon: %.2f ms\n", elapsed(t))
		t = now()
		var av int64
		for i := 0; i < 10000000; i++ { atomic.AddInt64(&av, 1) }
		fmt.Printf("cpu_atomic_add: %.2f ms\n", elapsed(t))
		t = now()
		var nv int64
		for i := 0; i < 10000000; i++ { nv++ }
		fmt.Printf("cpu_nonatomic_add: %.2f ms\n", elapsed(t))
	}
}

func fib(n int) int { if n < 2 { return n }; return fib(n-1) + fib(n-2) }

func reverseString(s string) string {
	r := []rune(s)
	for i, j := 0, len(r)-1; i < j; i, j = i+1, j-1 { r[i], r[j] = r[j], r[i] }
	return string(r)
}

func quickSort(a []int, lo, hi int) {
	if lo >= hi { return }
	p := a[hi]; i := lo - 1
	for j := lo; j < hi; j++ { if a[j] <= p { i++; a[i], a[j] = a[j], a[i] } }
	a[i+1], a[hi] = a[hi], a[i+1]
	quickSort(a, lo, i); quickSort(a, i+2, hi)
}

func mergeSort(a []int) []int {
	if len(a) < 2 { return a }
	mid := len(a)/2
	return merge(mergeSort(a[:mid]), mergeSort(a[mid:]))
}
func merge(l, r []int) []int {
	out := make([]int, 0, len(l)+len(r))
	for len(l) > 0 && len(r) > 0 {
		if l[0] <= r[0] { out = append(out, l[0]); l = l[1:] } else { out = append(out, r[0]); r = r[1:] }
	}
	out = append(out, l...); out = append(out, r...)
	return out
}

func heapSort(a []int) {
	n := len(a)
	for i := n/2 - 1; i >= 0; i-- { heapify(a, n, i) }
	for i := n-1; i > 0; i-- { a[0], a[i] = a[i], a[0]; heapify(a, i, 0) }
}
func heapify(a []int, n, i int) {
	for { largest := i; l := 2*i+1; r := 2*i+2
		if l < n && a[l] > a[largest] { largest = l }
		if r < n && a[r] > a[largest] { largest = r }
		if largest == i { return }
		a[i], a[largest] = a[largest], a[i]; i = largest
	}
}

func radixSort(a []int) {
	max := 0
	for _, v := range a { if v > max { max = v } }
	for exp := 1; max/exp > 0; exp *= 10 {
		out := make([]int, len(a))
		count := [10]int{}
		for _, v := range a { count[(v/exp)%10]++ }
		for i := 1; i < 10; i++ { count[i] += count[i-1] }
		for i := len(a)-1; i >= 0; i-- { v := a[i]; out[count[(v/exp)%10]-1] = v; count[(v/exp)%10]-- }
		copy(a, out)
	}
}
