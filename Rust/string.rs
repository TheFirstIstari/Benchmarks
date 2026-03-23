const STR_LEN: usize = 1000;
const ITERATIONS: usize = 100_000;

fn hash_djb2(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for c in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(c as u32);
    }
    hash
}

fn count_splits(s: &str, delim: char) -> usize {
    s.chars().filter(|&c| c == delim).count() + 1
}

fn main() {
    println!("Rust String Operations ({} iterations)", ITERATIONS);
    
    let test_str: String = (0..STR_LEN)
        .map(|i| (b'a' + (i % 26) as u8) as char)
        .collect();
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        hash_djb2(&test_str);
    }
    let t1 = std::time::Instant::now();
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    println!("Hash: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _ = test_str.chars().rev().collect::<String>();
    }
    let t1 = std::time::Instant::now();
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    println!("Reverse: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        count_splits(&test_str, ',');
    }
    let t1 = std::time::Instant::now();
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    println!("Split: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
}
