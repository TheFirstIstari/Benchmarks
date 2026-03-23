use std::time::Instant;
use std::alloc::{alloc, dealloc, Layout};
use std::thread;
use std::sync::{Arc, Mutex};

const BLOCK_SIZE: usize = 64;
const LIVE_BLOCKS: usize = 1024;
const TOTAL_ALLOCS: usize = 10_000_000;
const NUM_THREADS: usize = 4;

fn allocate_and_free() {
    let mut blocks: Vec<*mut u8> = Vec::with_capacity(LIVE_BLOCKS);
    let layout = Layout::from_size_align(BLOCK_SIZE, 64).unwrap();
    let per_thread = TOTAL_ALLOCS / NUM_THREADS;
    
    for _ in 0..per_thread {
        let ptr = unsafe { alloc(layout) };
        blocks.push(ptr);
        
        if blocks.len() >= LIVE_BLOCKS {
            for &ptr in blocks[..LIVE_BLOCKS / 2].iter() {
                unsafe { dealloc(ptr, layout) };
            }
            blocks.drain(0..LIVE_BLOCKS / 2);
        }
    }
    
    for ptr in blocks {
        unsafe { dealloc(ptr, layout) };
    }
}

fn main() {
    println!("Rust Memory Allocator ({} threads, {} blocks, {} total allocs)",
        NUM_THREADS, LIVE_BLOCKS, TOTAL_ALLOCS);
    
    let t0 = Instant::now();
    let handles: Vec<_> = (0..NUM_THREADS)
        .map(|_| thread::spawn(allocate_and_free))
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    let t1 = Instant::now();
    
    let ms = t1.duration_since(t0).as_secs_f64() * 1000.0;
    let allocs_per_sec = TOTAL_ALLOCS as f64 / (ms / 1000.0);
    
    println!("Total time: {:.2} ms", ms);
    println!("Allocations/sec: {:.0}", allocs_per_sec);
    println!("Avg latency: {:.2} ns/allocation", (ms * 1e6) / TOTAL_ALLOCS as f64);
}
