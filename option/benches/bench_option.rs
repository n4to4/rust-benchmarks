use bench_option::*;
use criterion::{criterion_group, criterion_main, Criterion};

// https://pkolaczk.github.io/overhead-of-optional/

/*
$ cargo bench

sum_simple              time:   [300.34 us 301.65 us 303.20 us]
sum_optional            time:   [300.21 us 302.01 us 303.96 us]
sum_optional_non_zero   time:   [300.06 us 301.36 us 302.91 us]
sum_boxed               time:   [254.28 us 256.33 us 259.17 us]
sum_iterator            time:   [310.95 us 312.14 us 313.52 us]
*/

/*
$ RUSTFLAGS="-C target-cpu=native" cargo bench

sum_simple              time:   [98.047 us 98.678 us 99.420 us]
sum_optional            time:   [99.428 us 99.832 us 100.25 us]
sum_optional_non_zero   time:   [100.00 us 100.52 us 101.17 us]
sum_boxed               time:   [581.05 us 583.46 us 586.09 us]
sum_iterator            time:   [104.53 us 104.88 us 105.28 us]
*/

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum_simple", |b| b.iter(sum_simple));
    c.bench_function("sum_optional", |b| b.iter(sum_optional));
    c.bench_function("sum_optional_non_zero", |b| b.iter(sum_optional_non_zero));
    c.bench_function("sum_boxed", |b| b.iter(sum_boxed));
    c.bench_function("sum_iterator", |b| b.iter(sum_iterator));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
