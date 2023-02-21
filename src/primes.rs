
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use std::cmp::{min, max};


fn isqrt(n: usize) -> usize {
    (n as f64).sqrt() as usize + 1
}

#[pyclass]
pub struct PrimeSieve {
    primes: Vec<usize>,
    index: usize
}


// Calculates primes up to n using sieve. O(n) memory required
fn sieve0(n: usize) -> Vec<usize> {
    let n = max(n, 4);
    let mut state = vec![true; n];
    state[0] = false; // not prime
    state[1] = false; // not prime

    let m = isqrt(n);
    for i in 2..=m {
        if state[i] {
            for j in (i * i..n).step_by(i) {
                state[j] = false;
            }
        }
    }
    state.iter().enumerate().filter(|(_, &s)| s).map(|(i, _)| i).collect::<Vec<_>>()
}


// Calculates primes up to n using seqmented sieve.
fn sieve(n: usize) -> Vec<usize> {
    let n = max(n, 4);
    let chunk_size = min(n, 100_000_000);
    // getting far more than required for small n
    let mut primes = sieve0(chunk_size);
    for n0 in (chunk_size..n).step_by(chunk_size) {
        let n1 = min(n0 + chunk_size, n);
        let m = isqrt(n1);
        let mut state = vec![true; n1 - n0];
        for p in &primes {
            if p > &m {
                break;
            }
            let s = match n0 % p {
                0 => p * (n0 / p),
                _ => p * (n0 / p + 1)
            };
            for i in (s..n1).step_by(*p) {
                state[i - n0] = false;
            }
        }
        primes.extend(
            state.iter().enumerate().filter(|(_, &s)| s).map(|(i, _)| n0 + i) //.collect::<Vec<_>>()
        );
    }
    primes
}


#[pymethods]
impl PrimeSieve {
    #[new]
    fn new(n: usize) -> Self {
        PrimeSieve{ primes: sieve(n), index: 0 }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<usize> {
        slf.index += 1;
        match slf.index {
            i if i <= slf.primes.len() => Some(slf.primes[i - 1]),
            _ => None
        }
    }
}


fn is_prime(n: usize, primes_below: &Vec::<usize>) -> bool {
    match n {
        0..=1 => false,
        2..=3 => true,
        _ => {
            for p in primes_below {
                if n % p == 0 {
                    return false;
                }
                // only need to go as far as sqrt(n)
                if p * p > n {
                    break;
                }
            }
            true
        }
    }
}


#[pyclass]
pub struct PrimeGenerator {
    found: Vec<usize>
}


#[pymethods]
impl PrimeGenerator {
    #[new]
    fn new() -> Self {
        PrimeGenerator{ found: vec![2, 3] }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<usize> {
        let mut n = *slf.found.last().unwrap();
        loop {
            n += 2;
            if is_prime(n, &slf.found) {
                slf.found.push(n);
                return Some(n);
            }
        }
    }
}


#[pyfunction]
pub fn nth_prime(n: usize) -> PyResult<usize> {
    if n == 0 {
        return Err(PyValueError::new_err("n must be >0"));
    }
    let mut found = Vec::with_capacity(n);
    found.push(2);
    found.push(3);
    while n > found.len() {
        let mut c = *found.last().unwrap();
        loop {
            c += 2;
            if is_prime(c, &found) {
                found.push(c);
                break;
            }
        }
    }
    Ok(found[n-1])
}


#[pyclass]
pub struct PrimeRange {
    index: usize,
    n: usize,
    seed_primes: Vec<usize>,
}


#[pymethods]
impl PrimeRange {
    #[new]
    fn new(m: usize, n: usize) -> Self {
        PrimeRange{ index: if m % 2 == 0 { m + 1 } else { m }, n, seed_primes: sieve(isqrt(n))}
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<usize> {
        while slf.index <= slf.n && !is_prime(slf.index, &slf.seed_primes) {
            slf.index += 2;
        }
        let ret = slf.index;
        slf.index += 2;
        match ret {
            r if r > slf.n => None,
            _ => Some(ret)
        }
    }
}


#[pyfunction(name="is_prime")]
pub fn is_prime_py(n: usize) -> bool {
    is_prime(n, &sieve(max(isqrt(n), 10)))
}


#[pyfunction]
pub fn prime_factors(n: usize) -> PyResult<Vec<usize>> {
    if n == 0 {
        return Err(PyValueError::new_err("input must be >=1"));
    }
    let mut factors = vec![];
    let mut m = n;
    for p in sieve(isqrt(n)) {
        while m % p == 0 {
            m /= p;
            factors.push(p);
        }
    }
    if m > 1 {
        factors.push(m);
    }
    Ok(factors)
}


