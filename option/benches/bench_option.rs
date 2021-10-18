use bench_option::*;
use criterion::{criterion_group, criterion_main, Criterion};

// https://pkolaczk.github.io/overhead-of-optional/

/*
$ cargo bench

sum_simple              time:   [328.49 us 331.87 us 335.22 us]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe

sum_optional            time:   [331.23 us 332.25 us 333.35 us]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

sum_optional_non_zero   time:   [331.47 us 332.44 us 333.55 us]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

sum_boxed               time:   [264.97 us 268.34 us 272.17 us]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe
*/

/*
$ RUSTFLAGS="-C target-cpu=native" cargo bench

sum_simple              time:   [109.04 us 109.34 us 109.67 us]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

sum_optional            time:   [109.04 us 109.31 us 109.58 us]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

sum_optional_non_zero   time:   [110.15 us 110.59 us 111.09 us]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

sum_boxed               time:   [593.40 us 595.43 us 597.77 us]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
*/

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum_simple", |b| b.iter(sum_simple));
    c.bench_function("sum_optional", |b| b.iter(sum_optional));
    c.bench_function("sum_optional_non_zero", |b| b.iter(sum_optional_non_zero));
    c.bench_function("sum_boxed", |b| b.iter(sum_boxed));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
