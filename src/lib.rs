use malachite::natural::{exhaustive, Natural};
use std::thread;

// the current algorithm is naive
// it assumes splitting the numbers evenly will give the threads an equal amount of work
// however multiplying larger numbers takes longer
// to evenly divide the work, we could use a sliding window approach for larger numbers
// where each thread calculates the product of 10_000 numbers, then is assigned the next batch of numbers
// for now the naive algorithm achieves a 2x speedup when using 8 cores compared to 1 core
const NUM_CORES: u8 = 8;

/// Calculate the factorial of a number, splitting the calculations across multiple threads
pub fn parallel_factorial(n: u64) -> Natural {
    let nums_per_thread = n / NUM_CORES as u64; // note integer division

    // create the threads, collect them all into a vector so all the threads are spawned and running
    let product_calculation_threads: Vec<_> = (0..NUM_CORES)
        .map(|thread_num| thread::spawn(move || calc_product(thread_num as u64, nums_per_thread)))
        .collect();

    // join the threads and accumulate the results
    let thread_product: Natural = product_calculation_threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .product();

    // multiply by any number cut off at the end (because of the integer division by NUM_CORES)
    let final_parts: Natural = range_product(nums_per_thread * (NUM_CORES as u64) + 1, n);
    thread_product * final_parts
}

fn calc_product(offset: u64, num_to_multiply: u64) -> Natural {
    let start = offset * num_to_multiply + 1; // add one to avoid multiplying by zero when offset = 0
    let end = (offset + 1) * num_to_multiply;
    range_product(start, end)
}

fn range_product(from: u64, to: u64) -> Natural {
    if from > to {
        1u8.into()
    } else {
        exhaustive::exhaustive_natural_inclusive_range(from.into(), to.into()).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fac_zero() {
        assert_eq!(parallel_factorial(0), 1)
    }

    #[test]
    fn test_fac_one() {
        assert_eq!(parallel_factorial(1), 1);
    }

    #[test]
    fn test_fac_small() {
        assert_eq!(parallel_factorial(2), 2);
        assert_eq!(parallel_factorial(3), 6);
        assert_eq!(parallel_factorial(4), 24);
        assert_eq!(parallel_factorial(5), 120);
    }

    fn fac(n: u128) -> u128 {
        (1..=n).product()
    }

    // all numbers that don't overflow u128s
    #[test]
    fn test_fac_large() {
        for i in 6..35 {
            println!("calculating fac of {i}");
            assert_eq!(parallel_factorial(i), fac(i as u128))
        }
    }
}
