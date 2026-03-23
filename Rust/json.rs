const ITERATIONS: usize = 5000;

fn main() {
    println!("Rust JSON Benchmark ({} iterations)", ITERATIONS);
    
    let test_json = r#"{"name":"test","age":42,"active":true,"scores":[1,2,3,4,5,6,7,8,9,10]}"#;
    let large_str = r#"{"data": ""#.to_string() + &"x".repeat(1000000) + r#""}"#;
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _copy = test_json.to_string();
    }
    let t1 = std::time::Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Parse: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = std::time::Instant::now();
    for i in 0..ITERATIONS {
        let _result = format!(r#"{{"id":{},"name":"user","value":{},"items":[1,2,3,4,5,6,7,8,9,10]}}"#, i, i as f64 / 10.0);
    }
    let t1 = std::time::Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Serialize: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let _pos = large_str.find(r#"{"#);
    }
    let t1 = std::time::Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Search: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = std::time::Instant::now();
    for _ in 0..ITERATIONS {
        let count = large_str.matches("\"\":").count();
    }
    let t1 = std::time::Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Count fields: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
}
