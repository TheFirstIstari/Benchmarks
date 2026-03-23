const ITERATIONS: usize = 10_000_000;

fn main() {
    println!("Rust Math Benchmark ({} iterations)", ITERATIONS);
    
    let mut result: f64 = 0.0;
    let t0 = std::time::Instant::now();
    for i in 1..ITERATIONS {
        let fi = i as f64;
        result += fi.sqrt() + fi.sin() + fi.cos();
    }
    let t1 = std::time::Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Trig functions: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    if result == 0.0 { println!(""); }
    
    result = 0.0;
    let t0 = std::time::Instant::now();
    for i in 1..ITERATIONS {
        let fi = i as f64;
        result += fi.ln() + (i as f64 % 10.0).exp();
    }
    let t1 = std::time::Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Exp/log functions: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    if result == 0.0 { println!(""); }
    
    result = 0.0;
    let t0 = std::time::Instant::now();
    for i in 1..ITERATIONS {
        let x = i as f64;
        result += x * x + x * x * x + (x * x + 1.0).sqrt();
    }
    let t1 = std::time::Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Arithmetic: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    if result == 0.0 { println!(""); }
}
