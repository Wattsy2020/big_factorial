use big_factorial::parallel_factorial;

fn main() {
    // 1_000_000 can execute in 5 seconds, 0.2 seconds on --release build
    // 10_000_000 can execute in 2.8 seconds on --release
    let n: u64 = std::env::args().nth(1).unwrap().parse().unwrap();

    let large_fac = parallel_factorial(n);
    let (mantissa, exponent, _) = large_fac
        .sci_mantissa_and_exponent_round::<f64>(malachite::rounding_modes::RoundingMode::Floor)
        .unwrap();
    println!("{n}! = {mantissa}^{exponent}");
}
