This is a small library and CLI tool for calculating large factorials quickly

The CLI tool has documentation accessible using the `--help` flag. 
In short, it allows you to set the number of threads for the computation, and whether to show full output
```shell
> big_factorial 10
10! = 1.7303466796875*2^21
> big_factorial 10 -f
10! = 3628800
```

The library can be used with any integer-like type that implements addition and multiplication
```rust
use big_factorial::{factorial, parallel_factorial};
assert_eq!(factorial::<u64>(4), 24);
assert_eq!(parallel_factorial::<u64>(4, 8), 24);
parallel_factorial::<malachite::Natural>(1_000_000, 8); // very large result
```
