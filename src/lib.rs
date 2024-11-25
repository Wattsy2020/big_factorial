//! Library for calculating factorials
//!
//! The primary function is `parallel_factorial`, which splits the factorial computation across multiple threads
//!
//! There is also `factorial`, which calculates the factorial using a single thread

use std::ops::Mul;
use rayon::prelude::*;

const WINDOW_SIZE: usize = 10_000;

/// Calculate the factorial of a number, splitting the calculations across multiple threads
///
/// # Examples
///
/// ```
/// use big_factorial::parallel_factorial;
/// assert_eq!(parallel_factorial::<u64>(4), 24);
/// ```
pub fn parallel_factorial<T>(n: u32) -> T
where
    T: From<u32> + Mul<T, Output = T> + Send + 'static,
{
    (1..n+1)
        .into_par_iter()
        .fold_chunks(WINDOW_SIZE, || 1.into(), |acc: T, x| acc * x.into())
        .reduce(|| 1.into(), |acc, x| acc * x)
}


/// Product of the numbers in [from, to] (i.e. inclusive product)
fn range_product<T>(from: u32, to: u32) -> T
where
    T: From<u32> + Mul<T, Output = T>,
{
    (from..=to).fold(1.into(), |acc: T, x| acc * x.into())
}

/// Calculate the factorial of a number, using a single thread
///
/// # Examples
///
/// ```
/// use big_factorial::factorial;
/// assert_eq!(factorial::<u64>(4), 24);
/// ```
pub fn factorial<T>(n: u32) -> T
where
    T: From<u32> + Mul<T, Output = T>,
{
    range_product(1, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parallel_fac(n: u32) -> u128 {
        parallel_factorial(n)
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
