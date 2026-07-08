use rand::Rng;
use std::hint::black_box;

const N: usize = 5_000_000;
const INSERTION_CUTOFF: usize = 32;

fn quicksort_opt(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 { return; }
    quicksort_inner(arr, 0, len - 1);
}

fn quicksort_inner(arr: &mut [i32], lo: usize, hi: usize) {
    let mut lo = lo;
    let mut hi = hi;
    while hi - lo > INSERTION_CUTOFF {
        // Median of three
        let mid = lo + (hi - lo) / 2;
        if arr[lo] > arr[mid] { arr.swap(lo, mid); }
        if arr[lo] > arr[hi] { arr.swap(lo, hi); }
        if arr[mid] > arr[hi] { arr.swap(mid, hi); }
        arr.swap(mid, hi);
        let pivot = arr[hi];
        let mut i = lo;
        for j in lo..hi {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, hi);
        // Tail recursion elimination: recurse on smaller side
        if i == 0 {
            lo = i + 1;
        } else if i - lo < hi - i {
            quicksort_inner(arr, lo, i - 1);
            lo = i + 1;
        } else {
            quicksort_inner(arr, i + 1, hi);
            hi = i - 1;
        }
    }
    // Insertion sort for small partitions
    for i in (lo + 1)..=hi {
        let key = arr[i];
        let mut j = i;
        while j > lo && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

// LSD radix sort for i32 — O(n) for fixed-width keys
fn radix_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut temp = vec![0i32; n];
    let mut src = &mut *arr as &mut [i32];
    let mut dst = &mut temp[..];

    for shift in (0..32).step_by(8) {
        let mut count = [0usize; 256];
        for &val in src.iter() {
            let key = if shift == 24 {
                ((val >> 24) as u8) ^ 0x80
            } else {
                ((val as u32) >> shift) as u8
            };
            count[key as usize] += 1;
        }
        let mut prefix = [0usize; 256];
        let mut sum = 0;
        for i in 0..256 {
            prefix[i] = sum;
            sum += count[i];
        }
        for &val in src.iter() {
            let key = if shift == 24 {
                ((val >> 24) as u8) ^ 0x80
            } else {
                ((val as u32) >> shift) as u8
            };
            dst[prefix[key as usize]] = val;
            prefix[key as usize] += 1;
        }
        std::mem::swap(&mut src, &mut dst);
    }
    if src.as_ptr() != arr.as_ptr() {
        arr.copy_from_slice(&temp);
    }
}

fn main() {
    println!("Rust Sort Benchmark (N={}, 5 iterations)", N);

    let mut rng = rand::thread_rng();
    let original: Vec<i32> = (0..N).map(|_| rng.gen::<i32>()).collect();

    let mut arr = original.clone();
    let t0 = std::time::Instant::now();
    arr.sort();
    let t1 = std::time::Instant::now();
    println!("Sort (stdlib): {:.2} ms ({:.0} elements/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        N as f64 / (t1 - t0).as_secs_f64());
    black_box(&arr);

    let mut arr = original.clone();
    let t0 = std::time::Instant::now();
    quicksort_opt(&mut arr);
    let t1 = std::time::Instant::now();
    println!("Quicksort: {:.2} ms ({:.0} elements/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        N as f64 / (t1 - t0).as_secs_f64());
    black_box(&arr);

    let mut arr = original.clone();
    let t0 = std::time::Instant::now();
    arr.sort_unstable();
    let t1 = std::time::Instant::now();
    println!("Sort_unstable: {:.2} ms ({:.0} elements/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        N as f64 / (t1 - t0).as_secs_f64());
    black_box(&arr);

    let mut arr = original.clone();
    let t0 = std::time::Instant::now();
    radix_sort(&mut arr);
    let t1 = std::time::Instant::now();
    println!("Radixsort: {:.2} ms ({:.0} elements/sec)",
        (t1 - t0).as_secs_f64() * 1000.0,
        N as f64 / (t1 - t0).as_secs_f64());
    black_box(&arr);
}
