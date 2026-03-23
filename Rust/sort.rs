use rand::Rng;

const N: usize = 5_000_000;
const ITERATIONS: usize = 5;

fn quicksort(arr: &mut [i32], low: isize, high: isize) {
    if low >= high { return; }
    let pivot = arr[high as usize];
    let mut i = low - 1;
    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize);
    quicksort(arr, low, i);
    quicksort(arr, i + 2, high);
}

fn main() {
    println!("Rust Sort Benchmark (N={}, {} iterations)", N, ITERATIONS);
    
    let mut rng = rand::thread_rng();
    let original: Vec<i32> = (0..N).map(|_| rng.gen::<i32>()).collect();
    
    let mut arr = original.clone();
    let t0 = std::time::Instant::now();
    arr.sort();
    let t1 = std::time::Instant::now();
    println!("Sort (stdlib): {:.2} ms ({:.0} elements/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        N as f64 / (t1 - t0).as_secs_f64());
    
    let mut arr = original.clone();
    let len = arr.len() as isize - 1;
    let t0 = std::time::Instant::now();
    quicksort(&mut arr, 0, len);
    let t1 = std::time::Instant::now();
    println!("Quicksort: {:.2} ms ({:.0} elements/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        N as f64 / (t1 - t0).as_secs_f64());
    
    let mut arr = original.clone();
    let t0 = std::time::Instant::now();
    arr.sort_unstable();
    let t1 = std::time::Instant::now();
    println!("Sort_unstable: {:.2} ms ({:.0} elements/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        N as f64 / (t1 - t0).as_secs_f64());
}
