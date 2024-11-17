from __future__ import annotations

from math import log, sqrt
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Generator


def _isqrt(n: int) -> int:
    return int(sqrt(n)) + 1


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
    ext_primes: list[int] = []
    c = primes[-1]
    while True:
        c += 2
        if c * c > n:
            break
        if _is_prime(c, primes) and _is_prime(c, ext_primes):
            ext_primes.append(c)
    return ext_primes


def prime_sieve(n: int) -> Generator[int, None, None]:
    assert n > 1
    a = [False, False] + [True] * (n - 2)
    for i in range(2, _isqrt(n)):
        if a[i]:
            for j in range(i * i, n, i):
                a[j] = False
    yield from (i for i in range(n) if a[i])


class PrimeGenerator:
    def __init__(self) -> None:
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
    def __init__(self, m: int, n: int) -> None:
        self.index = m - 2 if m % 2 else m - 1
        self.n = n
        self.seed_primes = _seed_primes(n)

    def __iter__(self) -> PrimeRange:
        return self

    def __next__(self) -> int:
        while True:
            self.index += 2
            if self.index > self.n:
                raise StopIteration
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
    if n < 6:
        return [2, 3, 5, 7, 11, 13][n - 1]
    m = int(n * log(n) + n * log(log(n))) if n < 7022 else int(n * log(n) + n * (log(log(n)) - 0.9385))
    p = sieve(m)
    assert len(p) >= n, f"{n}: {m} ({len(p)}) {p[-1]}"
    return p[n - 1]


def prime_factors(n: int) -> list[int]:
    if n == 0:
        raise ValueError("input must be >=1")
    factors = []
    m: float = n
    for p in _seed_primes(n):
        while m % p == 0:
            m /= p
            factors.append(p)
    if m > 1:
        factors.append(int(m))
    return factors


# Calculates primes up to n using sieve. O(n) memory required
def sieve0(n: int) -> Generator[int, None, None]:
    n = max(n, 4)
    state = [True] * n
    state[0] = state[1] = False
    m = _isqrt(n)
    for i in range(2, m + 1):
        if state[i]:
            for j in range(i * i, n, i):
                state[j] = False
    yield from (i for i, p in enumerate(state) if p)


def sieve(n: int) -> list[int]:
    n = max(n, 4)
    chunk_size = min(n, 100_000_000)
    # getting far more than required for small n
    primes = list(sieve0(chunk_size))
    for n0 in range(chunk_size, n, chunk_size):
        n1 = min(n0 + chunk_size, n)
        m = _isqrt(n1)
        state = [True] * (n1 - n0)
        for p in primes:
            if p > m:
                break
            match n0 % p:
                case 0:
                    s = p * (n0 // p)
                case _:
                    s = p * (n0 // p + 1)
            for i in range(s, n1, p):
                state[i - n0] = False
        primes.extend(n0 + i for i, p in enumerate(state) if p)
    return primes


if __name__ == "__main__":
    for i in range(1, 8):
        print(10**i, nth_prime(10**i))
