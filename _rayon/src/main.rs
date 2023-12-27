/*
    Rayon is a library that helps you achieve parallelism in Rust.
    It is meant to parallelize iterators and other collection operations.
    Rayon should be used when you have CPU-bound tasks that can be parallelized easily.
    Rayon is not meant to be used when you have a lot of waiting I/O-bound tasks.
    For this, refer to Tokio.
*/

use std::{collections::HashMap, ops::Range, time::Instant};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
    let range = 0..52;
    println!("Calculating Fibonacchi numbers in range: {:?}", range);

    let mut now = Instant::now();
    sequential(range.clone());
    let sequential_ellapsed = now.elapsed();
    println!("Sequential: {:?}", sequential_ellapsed);

    now = Instant::now();
    parallel(range.clone());
    let parallel_ellapsed = now.elapsed();
    println!("Parallel: {:?}", parallel_ellapsed);
}

fn sequential(range: Range<u64>) -> HashMap<u64, u64> {
    range.map(|index| (index, fibonacci(index))).collect()
}

fn parallel(range: Range<u64>) -> HashMap<u64, u64> {
    range
        .into_par_iter()
        .map(|index| (index, fibonacci(index)))
        .collect()
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
