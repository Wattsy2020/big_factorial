This is a small library and CLI tool for calculating large factorials quickly

The CLI tool has documentation accessible using the `--help` flag. 
In short, it allows you to set the number of threads for the computation, and whether to show full output
```shell
> big_factorial 10
10! = 1.7303466796875*2^21
> big_factorial 10 -f
10! = 3628800
```

Using multiple threads speeds up computation significantly
```shell
> time target/release/big_factorial 1000000 --num-threads 1
1000000! = 1.7653664438571652*2^18488884
target/release/big_factorial 1000000 --num-threads 1  87.62s user 0.58s system 99% cpu 1:28.80 total
> time target/release/big_factorial 1000000 --num-threads 8
1000000! = 1.7653664438571652*2^18488884
target/release/big_factorial 1000000 --num-threads 8  3.04s user 0.09s system 143% cpu 2.184 total
```


The library can be used with any integer-like type that implements addition and multiplication
```rust
use big_factorial::{factorial, parallel_factorial};
assert_eq!(factorial::<u64>(4), 24);
assert_eq!(parallel_factorial::<u64>(4, 8), 24);
parallel_factorial::<malachite::Natural>(1_000_000, 8); // very large result
```
