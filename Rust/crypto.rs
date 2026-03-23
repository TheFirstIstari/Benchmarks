use std::time::Instant;

const ITERATIONS: usize = 1000;
const BLOCK_SIZE: usize = 16;

fn main() {
    println!("Rust Crypto Benchmark");
    
    let mut key = [0u8; 16];
    let mut input = [0xAAu8; 16];
    let mut output = [0u8; 16];
    
    for (i, k) in key.iter_mut().enumerate() {
        *k = i as u8;
    }
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        for i in 0..BLOCK_SIZE {
            output[i] = input[i] ^ key[i];
        }
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    
    println!("XOR (simulated AES): {:.2} ms", total);
    
    let mut checksum: i32 = 0;
    let t0 = Instant::now();
    for i in 0..ITERATIONS * 1000 {
        checksum += output[i % 16] as i32;
    }
    let total = t0.elapsed().as_secs_f64() * 1000.0;
    println!("Integer ops: {:.2} ms (checksum: {})", total, checksum);
}
