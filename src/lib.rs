//! Library for calculating factorials
//!
//! The primary function is `parallel_factorial`, which splits the factorial computation across multiple threads
//!
//! There is also `factorial`, which calculates the factorial using a single thread

use std::ops::Mul;
use std::thread;

/// Calculate the factorial of a number, splitting the calculations across multiple threads
///
/// # Examples
///
/// ```
/// use big_factorial::parallel_factorial;
/// assert_eq!(parallel_factorial::<u64>(4, 2), 24);
/// ```
pub fn parallel_factorial<T>(n: u64, num_threads: u8) -> T
where
    T: From<u64> + Mul<T, Output = T> + Send + 'static,
{
    let nums_per_thread = n / num_threads as u64; // note integer division

    // create the threads, collect them all into a vector so all the threads are spawned and running
    let product_calculation_threads: Vec<_> = (0..num_threads)
        .map(|thread_num| {
            thread::spawn(move || calc_product::<T>(thread_num as u64, nums_per_thread))
        })
        .collect();

    // join the threads and accumulate the results
    let thread_product: T = product_calculation_threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .fold(1.into(), |acc, x| acc * x);

    // multiply by any number cut off at the end (because of the integer division by NUM_CORES)
    let final_parts: T = range_product(nums_per_thread * (num_threads as u64) + 1, n);
    thread_product * final_parts
}

fn calc_product<T>(offset: u64, num_to_multiply: u64) -> T
where
    T: From<u64> + Mul<T, Output = T>,
{
    let start = offset * num_to_multiply + 1; // add one to avoid multiplying by zero when offset = 0
    let end = (offset + 1) * num_to_multiply;
    range_product(start, end)
}

/// Sum of the numbers in [from, to] (i.e. inclusive sum)
fn range_product<T>(from: u64, to: u64) -> T
where
    T: From<u64> + Mul<T, Output = T>,
{
    if from > to {
        1.into()
    } else {
        (from..=to).fold(1.into(), |acc: T, x| acc * x.into())
    }
}

/// Calculate the factorial of a number, using a single thread
///
/// # Examples
///
/// ```
/// use big_factorial::factorial;
/// assert_eq!(factorial::<u64>(4), 24);
/// ```
pub fn factorial<T>(n: u64) -> T
where
    T: From<u64> + Mul<T, Output = T>,
{
    range_product(1, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parallel_fac(n: u64) -> u128 {
        parallel_factorial(n, 8)
    }

    #[test]
    fn test_fac_zero() {
        assert_eq!(parallel_fac(0), 1)
    }

    #[test]
    fn test_fac_one() {
        assert_eq!(parallel_fac(1), 1);
    }

    #[test]
    fn test_fac_small() {
        assert_eq!(parallel_fac(2), 2);
        assert_eq!(parallel_fac(3), 6);
        assert_eq!(parallel_fac(4), 24);
        assert_eq!(parallel_fac(5), 120);
    }

    fn fac(n: u128) -> u128 {
        (1..=n).product()
    }

    // all numbers that don't overflow u128s
    #[test]
    fn test_fac_large() {
        for i in 6..35 {
            println!("calculating fac of {i}");
            assert_eq!(parallel_fac(i), fac(i as u128))
        }
    }
}
