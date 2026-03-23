use std::time::Instant;

const ITERATIONS: usize = 1_000_000;
const STR_LEN: usize = 1000;

fn hash_sdbm(s: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    for &c in s {
        hash = c as u32 + (hash << 6) + (hash << 16) - hash;
    }
    hash
}

fn hash_djb2(s: &[u8]) -> u32 {
    let mut hash: u32 = 5381;
    for &c in s {
        hash = ((hash << 5) + hash) + c as u32;
    }
    hash
}

fn hash_fnv(s: &[u8]) -> u32 {
    let mut hash: u32 = 2166136261;
    for &c in s {
        hash ^= c as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}

fn main() {
    println!("Rust Hashing Benchmark ({} iterations)", ITERATIONS);
    
    let data: Vec<u8> = vec![b'a'; STR_LEN];
    let mut result: u32 = 0;
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        result += hash_sdbm(&data);
    }
    let t1 = Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("SDBM: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        result += hash_djb2(&data);
    }
    let t1 = Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("DJB2: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        result += hash_fnv(&data);
    }
    let t1 = Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("FNV: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    if result == 0 { println!(""); }
}
