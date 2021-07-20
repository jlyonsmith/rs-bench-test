use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_bench_test::*;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("fib #1 35", |b| b.iter(|| fibonacci_1(black_box(35))));
  c.bench_function("fib #2 35", |b| b.iter(|| fibonacci_2(black_box(35))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
