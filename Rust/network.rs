use std::io::{Read, Write};
use std::time::Instant;

const ITERATIONS: usize = 100;
const BUFFER_SIZE: usize = 65536;

fn main() {
    println!("Rust Network Benchmark");
    
    let buffer: Vec<u8> = vec![b'A'; BUFFER_SIZE];
    let mut result = Vec::new();
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        result.clear();
        result.extend_from_slice(&buffer);
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    
    let throughput = (ITERATIONS * BUFFER_SIZE) as f64 / (total / 1000.0) / 1e9;
    println!("Vec append: {:.2} GB/s ({:.2} ms)", throughput, total);
    
    let data = vec![b'A'; 8192];
    let mut dest = vec![b'\0'; 8192];
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS * 10 {
        dest.copy_from_slice(&data);
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    
    let throughput = (ITERATIONS * 10 * 8192) as f64 / (total / 1000.0) / 1e9;
    println!("Memcpy 8K: {:.2} GB/s ({:.2} ms)", throughput, total);
}
