use std::thread;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

const NUM_THREADS: usize = 8;
const OPS_PER_THREAD: usize = 10_000_000;

fn main() {
    println!("Rust Concurrency Benchmark ({} threads, {} ops/thread)",
        NUM_THREADS, OPS_PER_THREAD);
    
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = std::time::Instant::now();
    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| {
            let c = counter.clone();
            thread::spawn(move || {
                for _ in 0..OPS_PER_THREAD {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for h in handles { h.join().unwrap(); }
    let t1 = std::time::Instant::now();
    let total_ops = NUM_THREADS * OPS_PER_THREAD;
    println!("Atomic: {:.2} ms ({:.0} ops/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        total_ops as f64 / (t1 - t0).as_secs_f64());
    
    let t0 = std::time::Instant::now();
    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|i| {
            let per_thread = OPS_PER_THREAD;
            thread::spawn(move || {
                let start = i * per_thread;
                let end = start + per_thread;
                let mut sum: u64 = 0;
                for j in start..end {
                    sum += (j as u64) * (j as u64);
                }
                sum
            })
        })
        .collect();
    let _sum: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    let t1 = std::time::Instant::now();
    println!("Parallel sum: {:.2} ms ({:.0} ops/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        total_ops as f64 / (t1 - t0).as_secs_f64());
    
    let t0 = std::time::Instant::now();
    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|i| {
            thread::spawn(move || {
                let start = i * OPS_PER_THREAD;
                let mut sum: u64 = 0;
                for j in start..start + OPS_PER_THREAD {
                    sum += (j as u64) * (j as u64);
                }
                sum
            })
        })
        .collect();
    let _sum: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    let t1 = std::time::Instant::now();
    println!("Thread pool: {:.2} ms ({:.0} ops/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        total_ops as f64 / (t1 - t0).as_secs_f64());
}
