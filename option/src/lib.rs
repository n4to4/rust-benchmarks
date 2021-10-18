use std::num::NonZeroU64;

pub const MAGIC: u64 = 7;

pub fn get_int(n: u64) -> u64 {
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

pub fn get_optional_int(n: u64) -> Option<u64> {
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

pub fn get_optional_non_zero(n: u64) -> Option<NonZeroU64> {
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

pub fn get_int_boxed(n: u64) -> Box<Option<u64>> {
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
