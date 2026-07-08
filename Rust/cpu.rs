use std::time::Instant;

const N: usize = 10_000_000;
const BRANCH_N: usize = 100_000_000;
const POINTER_N: usize = 50_000_000;
const SIMD_N: usize = 50_000_000;

fn fmt_ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

// 1. Integer arithmetic
fn bench_fibonacci() {
    let mut results = Vec::with_capacity(N);
    let t0 = Instant::now();
    for j in 0..N {
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 0..50 {
            let c = a.wrapping_add(b);
            a = b; b = c;
        }
        results.push(a ^ (j as u64));  // prevent DCE
    }
    let ms = fmt_ms(t0.elapsed());
    let sum: u64 = results.iter().sum();
    println!("cpu_fibonacci: {:.2} ms ({:.0} ops/sec)", ms, N as f64 / (ms / 1000.0));
    std::hint::black_box(sum);
    results.truncate(0);
}

fn bench_prime_sieve() {
    let limit = 10_000_000;
    let mut sieve = vec![false; limit + 1];
    let mut count = 0u64;
    let t0 = Instant::now();
    for i in 2..=limit {
        if !sieve[i] {
            count += 1;
            if (i as u64).wrapping_mul(i as u64) <= limit as u64 {
                let mut j = i.wrapping_mul(i);
                while j <= limit {
                    sieve[j] = true;
                    j += i;
                }
            }
        }
    }
    let ms = fmt_ms(t0.elapsed());
    println!("cpu_prime_sieve: {:.2} ms ({} primes)", ms, count);
}

fn bench_popcount() {
    let data: Vec<u32> = (0..N as u32).map(|i| i.wrapping_mul(0x9e3779b9)).collect();
    let mut total = 0u64;
    let t0 = Instant::now();
    for &x in &data {
        total += x.count_ones() as u64;
    }
    let ms = fmt_ms(t0.elapsed());
    println!("cpu_popcount: {:.2} ms ({:.0} ops/sec)", ms, N as f64 / (ms / 1000.0));
    std::hint::black_box(total);
}

// 2. Branch prediction
fn bench_branch_prediction() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let unsorted: Vec<i64> = (0..BRANCH_N).map(|_| rng.gen_range(0..100)).collect();
    let mut sorted = unsorted.clone();
    sorted.sort();

    let t0 = Instant::now();
    let mut sum_unsorted: i64 = 0;
    for &x in &unsorted {
        if x > 50 { sum_unsorted += x; }
    }
    let ms_unsorted = fmt_ms(t0.elapsed());
    println!("cpu_branch_unsorted: {:.2} ms", ms_unsorted);

    let t0 = Instant::now();
    let mut sum_sorted: i64 = 0;
    for &x in &sorted {
        if x > 50 { sum_sorted += x; }
    }
    let ms_sorted = fmt_ms(t0.elapsed());
    println!("cpu_branch_sorted: {:.2} ms ({:.1}x faster)", ms_sorted, ms_unsorted / ms_sorted);

    std::hint::black_box((sum_unsorted, sum_sorted));
}

// 3. Memory latency (pointer chasing)
fn bench_memory_latency() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..POINTER_N).collect();
    // Fisher-Yates shuffle
    for i in (1..POINTER_N).rev() {
        let j = rng.gen_range(0..=i);
        indices.swap(i, j);
    }

    let t0 = Instant::now();
    let mut pos = 0usize;
    for _ in 0..POINTER_N {
        pos = indices[pos];
    }
    let ms = fmt_ms(t0.elapsed());
    println!("cpu_memory_latency: {:.2} ms ({:.0} traversals/sec)", ms, POINTER_N as f64 / (ms / 1000.0));
    std::hint::black_box(pos);
}

// 4. SIMD — use explicit NEON on aarch64
#[cfg(target_arch = "aarch64")]
fn bench_simd() {
    use std::arch::aarch64::*;
    let mut a = vec![0.0f32; SIMD_N];
    let mut b = vec![0.0f32; SIMD_N];
    let mut c = vec![0.0f32; SIMD_N];
    for i in 0..SIMD_N { a[i] = i as f32; b[i] = (i % 100) as f32; }

    let t0 = Instant::now();
    unsafe {
        for i in (0..SIMD_N).step_by(4) {
            let va = vld1q_f32(a.as_ptr().add(i));
            let vb = vld1q_f32(b.as_ptr().add(i));
            let vc = vaddq_f32(va, vb);
            let vd = vmulq_f32(vc, va);
            vst1q_f32(c.as_mut_ptr().add(i), vd);
        }
    }
    let ms = fmt_ms(t0.elapsed());
    let mut total = 0.0f32;
    for i in (0..1000) { total += c[i * 1000]; }
    println!("cpu_simd_neon: {:.2} ms ({:.0} ops/sec)", ms, (SIMD_N as f64 / 4.0) / (ms / 1000.0));
    std::hint::black_box(total);
}

#[cfg(not(target_arch = "aarch64"))]
fn bench_simd() {
    let a: Vec<f32> = (0..SIMD_N).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..SIMD_N).map(|i| (i % 100) as f32).collect();
    let mut c = vec![0.0f32; SIMD_N];

    let t0 = Instant::now();
    for i in 0..SIMD_N {
        c[i] = (a[i] + b[i]) * a[i];
    }
    let ms = fmt_ms(t0.elapsed());
    let mut total = 0.0f32;
    for i in (0..1000) { total += c[i * 1000]; }
    println!("cpu_simd_scalar: {:.2} ms ({:.0} ops/sec)", ms, SIMD_N as f64 / (ms / 1000.0));
    std::hint::black_box(total);
}

// 5. Atomic
fn bench_atomic() {
    use std::sync::atomic::{AtomicI64, Ordering};
    let atomic_val = AtomicI64::new(0);
    let t0 = Instant::now();
    for i in 0..N {
        atomic_val.fetch_add(std::hint::black_box(1), Ordering::Relaxed);
        std::hint::black_box(i);
    }
    let ms_atomic = fmt_ms(t0.elapsed());
    let atomic_final = atomic_val.load(Ordering::Relaxed);
    println!("cpu_atomic_add: {:.2} ms ({:.0} ops/sec)", ms_atomic, N as f64 / (ms_atomic / 1000.0));

    let mut regular_val = 0i64;
    let t0 = Instant::now();
    for i in 0..N {
        regular_val = regular_val.wrapping_add(1);
        std::hint::black_box(regular_val);
        std::hint::black_box(i);
    }
    let ms_reg = fmt_ms(t0.elapsed());
    println!("cpu_nonatomic_add: {:.2} ms ({:.0} ops/sec, {:.1}x faster)", ms_reg, N as f64 / (ms_reg / 1000.0), ms_atomic / ms_reg);

    std::hint::black_box((atomic_final, regular_val));
}

fn main() {
    println!("Rust CPU Benchmark");
    println!("Apple M4 — 10 cores (4P+6E), 16GB unified\n");

    println!("--- Integer Arithmetic ---");
    bench_fibonacci();
    bench_prime_sieve();
    bench_popcount();

    println!("\n--- Branch Prediction ---");
    bench_branch_prediction();

    println!("\n--- Memory Latency ---");
    bench_memory_latency();

    println!("\n--- SIMD Vectorization ---");
    bench_simd();

    println!("\n--- Atomic Operations ---");
    bench_atomic();
}
