use rand::Rng;

fn matmul(C: &mut [f64], A: &[f64], B: &[f64], n: usize) {
    for i in 0..n {
        for k in 0..n {
            let aik = A[i * n + k];
            for j in 0..n {
                C[i * n + j] += aik * B[k * n + j];
            }
        }
    }
}

fn transpose(T: &mut [f64], M: &[f64], n: usize) {
    for i in 0..n {
        for j in 0..n {
            T[j * n + i] = M[i * n + j];
        }
    }
}

fn add_matrices(C: &mut [f64], A: &[f64], B: &[f64], _n: usize) {
    for i in 0..C.len() {
        C[i] = A[i] + B[i];
    }
}

fn main() {
    const N: usize = 2000;
    const ITERATIONS: usize = 5;
    
    println!("Rust Matrix Benchmark ({}x{}, {} iterations)", N, N, ITERATIONS);
    
    let mut rng = rand::thread_rng();
    let A: Vec<f64> = (0..N * N).map(|_| rng.gen()).collect();
    let B: Vec<f64> = (0..N * N).map(|_| rng.gen()).collect();
    let mut C: Vec<f64> = vec![0.0; N * N];
    let mut T: Vec<f64> = vec![0.0; N * N];
    
    let mut total_ns: u64 = 0;
    let mut sum: f64 = 0.0;
    
    for _ in 0..ITERATIONS {
        C.fill(0.0);
        let t0 = std::time::Instant::now();
        matmul(&mut C, &A, &B, N);
        let t1 = std::time::Instant::now();
        total_ns += t1.duration_since(t0).as_nanos() as u64;
        sum += C.iter().sum::<f64>();
    }
    
    let avg_ms = (total_ns / ITERATIONS as u64) as f64 / 1e6;
    let ops = N as f64 * N as f64 * N as f64 * 2.0;
    let gflops = (ops * ITERATIONS as f64) / (total_ns as f64 / 1e9) / 1e9;
    
    println!("Multiply: {:.2} ms ({:.2} GFLOPS)", avg_ms, gflops);
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        transpose(&mut T, &A, N);
        let sum = T.iter().sum::<f64>();
        std::hint::black_box(sum);
    }
    let t1 = std::time::Instant::now();
    let avg_ms = (t1.duration_since(t0).as_nanos() as f64 / ITERATIONS as f64) / 1e6;
    println!("Transpose: {:.2} ms", avg_ms);
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        add_matrices(&mut C, &A, &B, N);
    }
    let t1 = std::time::Instant::now();
    let avg_ms = (t1.duration_since(t0).as_nanos() as f64 / ITERATIONS as f64) / 1e6;
    println!("Add: {:.2} ms", avg_ms);
}
