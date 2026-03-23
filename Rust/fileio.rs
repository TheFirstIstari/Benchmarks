use std::fs::{File};
use std::io::{Read, Write, BufRead, BufReader, Seek, SeekFrom};
use std::time::Instant;

const ITERATIONS: usize = 1000;
const FILE_SIZE: usize = 5000000;

fn main() {
    println!("Rust File I/O Benchmark ({} iterations)", ITERATIONS);
    
    let data: Vec<u8> = (0..FILE_SIZE).map(|i| b'a' + (i % 26) as u8).collect();
    
    let _ = std::fs::remove_file("/tmp/bench_io.dat");
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let mut f = File::create("/tmp/bench_io.dat").unwrap();
        f.write_all(&data).unwrap();
    }
    let t1 = Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Write: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let mut f = File::open("/tmp/bench_io.dat").unwrap();
        let mut buf = vec![0u8; FILE_SIZE];
        f.read_exact(&mut buf).unwrap();
    }
    let t1 = Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Read: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let f = File::open("/tmp/bench_io.dat").unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let _ = line;
        }
    }
    let t1 = Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Read lines: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let t0 = Instant::now();
    for _ in 0..ITERATIONS {
        let mut f = File::open("/tmp/bench_io.dat").unwrap();
        f.seek(SeekFrom::Start((FILE_SIZE / 2) as u64)).unwrap();
        let mut buf = vec![0u8; 1024];
        f.read(&mut buf).unwrap();
    }
    let t1 = Instant::now();
    let ms = (t1 - t0).as_secs_f64() * 1000.0;
    println!("Random access: {:.2} ms ({:.0} ops/sec)", 
        ms, ITERATIONS as f64 / (ms / 1000.0));
    
    let _ = std::fs::remove_file("/tmp/bench_io.dat");
}
