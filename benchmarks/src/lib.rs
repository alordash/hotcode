pub fn regular_slow_fibonacci(n: u128) -> u128 {
    slow_fibonacci(n)
}

#[hotcode::hotreload(always)]
pub fn hotreload_slow_fibonacci(n: u128) -> u128 {
    slow_fibonacci(n)
}

#[inline(always)]
fn slow_fibonacci(n: u128) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        n => slow_fibonacci(n - 1) + slow_fibonacci(n - 2),
    }
}

pub fn regular_fast_fibonacci(n: u128) -> u128 {
    fast_fibonacci(n)
}

#[hotcode::hotreload(always)]
pub fn hotreload_fast_fibonacci(n: u128) -> u128 {
    fast_fibonacci(n)
}

#[inline(always)]
fn fast_fibonacci(n: u128) -> u128 {
    let (mut a, mut b) = (0, 1);
    for _ in 1..=n {
        (a, b) = (a + b, a);
    }
    return a;
}
