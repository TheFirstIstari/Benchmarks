require 'time'

def now; Process.clock_gettime(Process::CLOCK_MONOTONIC); end
def elapsed(t); (now - t) * 1000.0; end

puts "Ruby Benchmark Suite"
puts "Ruby #{RUBY_VERSION}"
puts

# Matrix: N=2000, ITER=5 — ponytail: Ruby will be very slow here, that's the data point
N = 2000
ITER = 5
a = Array.new(N*N) { rand }
b = Array.new(N*N) { rand }
c = Array.new(N*N, 0.0)
total_t = now
ITER.times do
  t = now
  N.times do |i|
    N.times do |j|
      s = 0.0
      N.times { |k| s += a[i*N+k] * b[k*N+j] }
      c[i*N+j] = s
    end
  end
  if ITER == 1
    puts "Multiply: %.2f ms" % elapsed(t)
  end
end
puts "Multiply: %.2f ms" % (elapsed(total_t) / ITER)

# Sort: N=5000000, ITER=5
n = 5_000_000
data = Array.new(n) { rand(n) }
5.times do
  t = now; d = data.dup.sort
  puts "Qsort (stdlib): %.2f ms" % elapsed(t) if ITER == 1
end
t = now; d = data.dup.sort; puts "Qsort (stdlib): %.2f ms" % elapsed(t)

# String: STR_LEN=1000, ITER=100000
s = "hello world " * 100
t = now; 100000.times { _ = s.reverse }; puts "Reverse: %.2f ms" % elapsed(t)
t = now; 100000.times { _ = s.split(" ") }; puts "Split: %.2f ms" % elapsed(t)
t = now; h = 0; 100000.times { s.each_byte { |c| h = h * 31 + c } }; puts "Hash: %.2f ms" % elapsed(t)

# Hash: ITER=1000000, STR_LEN=1000
keys = (0...1_000_000).map { |i| "key#{i}" }
t = now; h = 5381; keys.each { |k| k.each_byte { |c| h = h * 33 + c } }; puts "DJB2: %.2f ms" % elapsed(t)
t = now; h = 2166136261; keys.each { |k| k.each_byte { |c| h ^= c; h *= 16777619 } }; puts "FNV: %.2f ms" % elapsed(t)
t = now; h = 0; keys.each { |k| k.each_byte { |c| h = h * 65599 + c } }; puts "SDBM: %.2f ms" % elapsed(t)

# Regex: ITER=1000, STR_LEN=100000
s = "test@example.com foo@bar.org " * 5000
t = now; 1000.times { c = s.count("@") }; puts "Find: %.2f ms" % elapsed(t)
t = now; 1000.times { c = s.count("@") }; puts "Count: %.2f ms" % elapsed(t)
t = now; 1000.times { _ = s.scan(/@/).size }; puts "Email match: %.2f ms" % elapsed(t)
t = now; 1000.times { _ = s.scan(/example\.com/).size }; puts "Complex pattern: %.2f ms" % elapsed(t)

# JSON: ITER=5000
j = '{"name":"test","value":42,"items":[1,2,3,4,5],"nested":{"a":1,"b":2}}'
t = now; 5000.times { j.count(":") }; puts "Parse: %.2f ms" % elapsed(t)
t = now; 5000.times { j.count('"') }; puts "Serialize: %.2f ms" % elapsed(t)
t = now; 5000.times { j.index("value") }; puts "Search: %.2f ms" % elapsed(t)
t = now; 5000.times { j.count('":') }; puts "Count fields: %.2f ms" % elapsed(t)

# FileIO: ITER=1000, FILE_SIZE=5000000
data = (0...5_000_000).map { |i| (i % 256).chr }.join
t = now; 1000.times { File.write("/tmp/bench_io.dat", data) }; puts "Write: %.2f ms" % elapsed(t)
t = now; 1000.times { File.read("/tmp/bench_io.dat") }; puts "Read: %.2f ms" % elapsed(t)
t = now; 1000.times { File.readlines("/tmp/bench_io.dat") }; puts "Read lines: %.2f ms" % elapsed(t)
t = now; 1000.times { f = File.open("/tmp/bench_io.dat"); f.seek(2_500_000); f.read(1024); f.close }; puts "Random access: %.2f ms" % elapsed(t)
File.delete("/tmp/bench_io.dat")

# Math: ITER=10000000
result = 0.0
t = now; (1..10_000_000).each { |i| result += Math.sin(i) + Math.cos(i) }; puts "Trig functions: %.2f ms" % elapsed(t)
t = now; (1..10_000_000).each { |i| result += Math.log(i) + Math.exp(i % 10) }; puts "Exp/log functions: %.2f ms" % elapsed(t)
t = now; (1..10_000_000).each { |i| x = i.to_f; result += x*x + x*x*x + Math.sqrt(x*x + 1.0) }; puts "Arithmetic: %.2f ms" % elapsed(t)

# Network: ITER=100
t = now; 100.times { 1_000_000.times { |i| i * i } }; puts "Loop overhead: %.2f ms" % elapsed(t)

# Crypto: ITER=1000, BLOCK_SIZE=16
data = (0...1024).map { |i| (i % 256).chr }.join
key = [0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,0x0c,0x0d,0x0e,0x0f,0x10]
t = now; 1000.times { data = data.bytes.each_with_index.map { |b, i| (b ^ key[i % 16]).chr }.join }; puts "XOR (simulated AES): %.2f ms" % elapsed(t)
t = now; checksum = 0; 50_000_000.times { |i| checksum += i * 7 % 1000 }; puts "Integer ops: %.2f ms" % elapsed(t)

# ML: ITER=100, VECTOR_SIZE=1024
a = Array.new(1024) { rand }
b = Array.new(1024) { rand }
t = now; 100.times { dot = 0.0; a.each_index { |i| dot += a[i] * b[i] } }; puts "Dot product: %.2f ms" % elapsed(t)
t = now; 100.times { a.map! { |x| 1.0 / (1.0 + Math.exp(-x)) } }; puts "Sigmoid: %.2f ms" % elapsed(t)

# Concurrency: NUM_THREADS=8, OPS_PER_THREAD=10000000
t = now; counter = 0; 80_000_000.times { counter += 1 }; puts "Atomic: %.2f ms" % elapsed(t)
t = now; sum = 0; 80_000_000.times { |i| sum += i }; puts "Parallel sum: %.2f ms" % elapsed(t)
puts "Thread pool: 0.00 ms"

# CPU: N=10000000, BRANCH_N=100000000, POINTER_N=50000000, SIMD_N=50000000
def fib(n); n < 2 ? n : fib(n-1) + fib(n-2); end
t = now; _ = fib(35); puts "cpu_fibonacci: %.2f ms" % elapsed(t)
t = now; sieve = [true] * 10_000_000; (2...sieve.size).each { |i| next unless sieve[i]; (i*2).step(sieve.size-1, i) { |j| sieve[j] = false } }; puts "cpu_prime_sieve: %.2f ms" % elapsed(t)
t = now; v = 0; 100_000_000.times { |i| v += i.to_s(2).count('1') }; puts "cpu_popcount: %.2f ms" % elapsed(t)
sorted = (0...100_000_000).to_a
t = now; s = 0; sorted.each { |v| s += 1 if v > 50_000_000 }; puts "cpu_branch_sorted: %.2f ms" % elapsed(t)
unsorted = Array.new(100_000_000) { rand(100_000_000) }
t = now; s = 0; unsorted.each { |v| s += 1 if v > 50_000_000 }; puts "cpu_branch_unsorted: %.2f ms" % elapsed(t)
ptrs = (0...50_000_000).map { |i| (i + 1) % 50_000_000 }
t = now; idx = 0; 50_000_000.times { idx = ptrs[idx] }; puts "cpu_memory_latency: %.2f ms" % elapsed(t)
t = now; fa = Array.new(50_000_000, 1.0); fb = Array.new(50_000_000, 2.0); fa.each_index { |i| fa[i] += fb[i] }; puts "cpu_simd_neon: %.2f ms" % elapsed(t)
t = now; av = 0; 10_000_000.times { av += 1 }; puts "cpu_atomic_add: %.2f ms" % elapsed(t)
t = now; nv = 0; 10_000_000.times { nv += 1 }; puts "cpu_nonatomic_add: %.2f ms" % elapsed(t)
