use std::time::Instant;

const ITERATIONS: usize = 1000;
const STR_LEN: usize = 100000;

fn main() {
    println!("Rust Regex Benchmark ({} iterations)", ITERATIONS);
    
    let test_str: String = (0..STR_LEN)
        .map(|i| (b'a' + (i % 26) as u8) as char)
        .collect();
    
    let re_find = regex::Regex::new(r"xyz").unwrap();
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = re_find.is_match(&test_str);
    }
    let t1 = Instant::now();
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    println!("Find: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let re_count = regex::Regex::new(r"[aeiou]").unwrap();
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = re_count.find_iter(&test_str).count();
    }
    let t1 = Instant::now();
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    println!("Count: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let re_email = regex::Regex::new(r"[a-z]+@[a-z]+\.[a-z]+").unwrap();
    let email_str = &test_str[..100];
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = re_email.is_match(email_str);
    }
    let t1 = Instant::now();
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    println!("Email match: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let re_complex = regex::Regex::new(r"[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = re_complex.is_match(&test_str);
    }
    let t1 = Instant::now();
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    println!("Complex pattern: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
}
