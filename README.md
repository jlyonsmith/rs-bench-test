# rs-bench-test

A quick test of Rust benchmarking using [Criterion](https:// github.com/bheisler/criterion.rs). Also shows how Rust can do tail recursion optimization for massive performance savings.

## Getting Started

1. [Install `Rust`](https://rustup.rs/)
2. `git clone` the repo
3. Run the benchmark with `cargo bench`

Observe the difference in performance between the regular, tail recursion optimized and looping version of the Fibonacci function.
