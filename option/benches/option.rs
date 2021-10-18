use criterion::{criterion_group, criterion_main, Criterion};
use std::num::NonZeroU64;

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

const MAGIC: u64 = 7;

fn get_int(n: u64) -> u64 {
    n & 0xFF
}

pub fn sum_simple() -> u64 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        let n = get_int(i);
        if n != MAGIC {
            sum += n;
        }
    }
    sum
}

fn get_optional_int(n: u64) -> Option<u64> {
    let i = n & 0xFF;
    if i == MAGIC {
        None
    } else {
        Some(i)
    }
}

pub fn sum_optional() -> u64 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        let value = get_optional_int(i);
        if let Some(k) = value {
            sum += k;
        }
    }
    sum
}

fn get_optional_non_zero(n: u64) -> Option<NonZeroU64> {
    let i = n & 0xFF;
    if i == MAGIC {
        None
    } else {
        NonZeroU64::new(i)
    }
}

pub fn sum_optional_non_zero() -> u64 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        let value = get_optional_non_zero(i);
        if let Some(k) = value {
            sum += k.get();
        }
    }
    sum
}

fn get_int_boxed(n: u64) -> Box<Option<u64>> {
    let opt = if n == MAGIC { None } else { Some(n) };
    Box::new(opt)
}

pub fn sum_boxed() -> u64 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        let n = get_int_boxed(i);
        if let Some(k) = *n {
            sum += k;
        }
    }
    sum
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum_simple", |b| b.iter(sum_simple));
    c.bench_function("sum_optional", |b| b.iter(sum_optional));
    c.bench_function("sum_optional_non_zero", |b| b.iter(sum_optional_non_zero));
    c.bench_function("sum_boxed", |b| b.iter(sum_boxed));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
