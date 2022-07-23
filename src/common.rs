pub const fn basis_blade_count(n: usize) -> usize {
    pow(2, n)
}

pub const fn binomial(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n - k))
}

// TODO: Maybe use [std::num::NonZeroUsize] as the return type?
pub const fn factorial(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub const fn pow(n: usize, mut k: usize) -> usize {
    let mut result = 1;
    while k > 0 {
        result *= n;
        k -= 1;
    }
    result
}

pub fn concat<const N: usize, const M: usize>(a: [i32; N], b: [i32; M]) -> [i32; N + M] {
    let mut result = [0i32; N + M];
    for i in 0..N {
        result[i] = a[i];
    }
    for i in 0..M {
        result[N + i] = b[i];
    }
    result
}

pub const fn dimension<const N: usize>(_: [i32; N]) -> usize {
    N
}

pub const fn even(n: usize) -> bool {
    n & 1 == 0
}

pub const fn odd(n: usize) -> bool {
    n & 1 != 0
}
