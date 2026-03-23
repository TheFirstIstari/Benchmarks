use std::time::Instant;

const ITERATIONS: usize = 100;
const VECTOR_SIZE: usize = 1024;

fn main() {
    println!("Rust ML Benchmark");
    
    let mut a: Vec<f32> = (0..VECTOR_SIZE).map(|i| i as f32 / VECTOR_SIZE as f32).collect();
    let mut b: Vec<f32> = (0..VECTOR_SIZE).map(|i| (VECTOR_SIZE - i) as f32 / VECTOR_SIZE as f32).collect();
    let mut c: Vec<f32> = vec![0.0; VECTOR_SIZE];
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        for i in 0..VECTOR_SIZE {
            c[i] = a[i] * b[i];
        }
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    println!("Element-wise mul: {:.2} ms", total);
    
    let t0 = Instant::now();
    let mut dot: f32 = 0.0;
    for _ in 0..ITERATIONS {
        dot = 0.0;
        for i in 0..VECTOR_SIZE {
            dot += a[i] * b[i];
        }
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    println!("Dot product: {:.2} ms (result: {:.4})", total, dot);
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        for i in 0..VECTOR_SIZE {
            a[i] = a[i] * 0.9 + b[i] * 0.1;
        }
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    println!("Lerp (mix): {:.2} ms", total);
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        for i in 0..VECTOR_SIZE {
            c[i] = 1.0 / (1.0 + a[i]);
        }
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    println!("Sigmoid: {:.2} ms", total);
}
