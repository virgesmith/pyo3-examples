from __future__ import annotations
from typing import Generator
from math import sqrt


def _is_prime(n: int, primes_below: list[int]) -> bool:
    if n < 4:
        return n in primes_below
    for p in primes_below:
        if n % p == 0:
            return False
        # only need to go as far as sqrt(n)
        if p * p > n:
            break
    return True

def _seed_primes(n: int) -> list[int]:
    primes = [2, 3]
    c = primes[-1]
    while True:
        c += 2
        if c * c > n:
            break
        if _is_prime(c, primes):
            primes.append(c)
    return primes


def _extend_seed_primes(primes: list[int], n: int) -> list[int]:
    ext_primes = []
    c = primes[-1]
    while True:
        c += 2
        if c * c > n:
            break
        if _is_prime(c, primes) and _is_prime(c, ext_primes):
            ext_primes.append(c)
    return ext_primes




def sieve(n: int) -> Generator[int, None, None]:
    assert n > 1
    a = [False, False] + [True] * (n - 2)
    for i in range(2, int(sqrt(n)) + 1):
        if a[i]:
            for j in range(i * i, n, i):
                a[j] = False
    yield from (i for i in range(n) if a[i])


class PrimeGenerator:
    def __init__(self):
        self.found = [2, 3]

    def __iter__(self) -> PrimeGenerator:
        return self

    def __next__(self) -> int:
        n = self.found[-1]
        while True:
            n += 2
            if _is_prime(n, self.found):
                self.found.append(n)
                return n

class PrimeRange:
    def __init__(self, m: int, n: int):
        self.index = m - 2 if m % 2 else m - 1
        self.n = n
        self.seed_primes = _seed_primes(n)

    def __iter__(self) -> PrimeGenerator:
        return self

    def __next__(self) -> int:
        # while self.index <= self.n and not _is_prime(self.index, self.seed_primes):
        #     self.index += 2
        # ret = self.index
        # self.index += 2
        # if ret > self.n:
        #     raise StopIteration()
        # return ret
        while True:
            self.index += 2
            if self.index > self.n:
                raise StopIteration()
            if _is_prime(self.index, self.seed_primes):
                break
        return self.index


def is_prime(n: int) -> bool:
    m = 1000000
    if n < m:
        return _is_prime(n, _seed_primes(n))
    primes = _seed_primes(m)
    while True:
        for p in primes:
            if n % p == 0:
                return False
        if m > n:
            break
        m *= 10
        primes = _extend_seed_primes(primes, m)
    return True


def nth_prime(n: int) -> int:
    if n < 1:
        raise ValueError("n must be >0")
    found = [2, 3]
    while n > len(found):
        c = found[-1]
        while True:
            c += 2
            if _is_prime(c, found):
                found.append(c)
                break
    return found[n-1]


def prime_factors(n: int) -> list[int]:
    if n == 0:
        raise ValueError("input must be >=1")
    factors = []
    m = n
    for p in _seed_primes(n):
        while m % p == 0:
            m /= p
            factors.append(p)
    if m > 1:
        factors.append(int(m))
    return factors


