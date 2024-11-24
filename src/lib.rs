//! Library for calculating factorials
//!
//! The primary function is `parallel_factorial`, which splits the factorial computation across multiple threads
//!
//! There is also `factorial`, which calculates the factorial using a single thread

use std::cmp::min;
use std::collections::HashSet;
use std::ops::Mul;
use std::sync::mpsc;
use std::thread;

const WINDOW_SIZE: u64 = 10_000;

struct ProductResult<T> {
    result: T,
    start_range: u64
}

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
    let mut result = 1.into();
    // number to calculate the next range from
    let mut start_range = 1;
    // the start of each range that we have issued to other threads for calculation
    let mut issued_ranges = HashSet::new();

    let (tx, rx) = mpsc::channel::<ProductResult<T>>();

    while start_range <= n || issued_ranges.len() > 0 {
        // start new threads to calculate results
        while start_range <= n && issued_ranges.len() < num_threads as usize {
            let tx_clone = tx.clone();
            let next_start_range = min(n + 1, start_range + WINDOW_SIZE);
            thread::spawn(move || {
                let result = range_product(start_range, next_start_range - 1);
                // discard error, parallel_factorial should keep the transmitter open until it's no longer needed, at which point failing to send is ok
                _ = tx_clone.send(ProductResult { result, start_range})
            });

            issued_ranges.insert(start_range);
            start_range = next_start_range;
        }

        // receive results
        let window_result = rx.recv().expect("receiver should be valid for the lifetime of the function");
        if issued_ranges.contains(&window_result.start_range) {
            issued_ranges.remove(&window_result.start_range);
            result = result * window_result.result;
        }

        // todo: handle failure later
        // check if an issued_range has been in progress for too long, if so spawn a new thread to calculate it
    }

    result
}


/// Product of the numbers in [from, to] (i.e. inclusive product)
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
