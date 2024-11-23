use clap::Parser;
use big_factorial::{factorial, parallel_factorial};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true, help="Number to calculate the factorial of")]
    x: u64,

    #[arg(short, long, default_value_t=1, help="Number of threads to use for the calculation")]
    num_threads: u8,

    #[arg(short, long, help="Show full output")]
    full_output: bool
}

fn main() {
    // 1_000_000 can execute in 5 seconds, 0.2 seconds on --release build
    // 10_000_000 can execute in 2.8 seconds on --release
    let args = Args::parse();

    let large_fac = if args.num_threads == 1 {
        factorial(args.x)
    } else {
        parallel_factorial(args.x, args.num_threads)
    };

    if args.full_output {
        println!("{large_fac}")
    } else {
        let (mantissa, exponent, _) = large_fac
            .sci_mantissa_and_exponent_round::<f64>(malachite::rounding_modes::RoundingMode::Floor)
            .unwrap();
        println!("{}! = {mantissa}^{exponent}", args.x);
    }
}
