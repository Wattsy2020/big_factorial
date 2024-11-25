use big_factorial::{factorial, parallel_factorial};
use clap::Parser;
use malachite::Natural;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true, help = "Number to calculate the factorial of")]
    x: u32,

    #[arg(short, long, help = "Use a single thread for the calculation")]
    single_threaded: bool,

    #[arg(short, long, help = "Show full output")]
    full_output: bool,
}

fn main() {
    // 1_000_000 can execute in 5 seconds, 0.2 seconds on --release build
    // 10_000_000 can execute in 2.8 seconds on --release
    let args = Args::parse();

    let large_fac: Natural = if args.single_threaded {
        factorial(args.x)
    } else {
        parallel_factorial(args.x)
    };

    if args.full_output {
        println!("{}! = {large_fac}", args.x)
    } else {
        let (mantissa, exponent, _) = large_fac
            .sci_mantissa_and_exponent_round::<f64>(malachite::rounding_modes::RoundingMode::Floor)
            .unwrap();
        println!("{}! = {mantissa}*2^{exponent}", args.x);
    }
}
