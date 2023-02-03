from functools import reduce
from operator import mul
import random
import pytest

from poetry_rust_integration import is_prime, nth_prime, prime_factors, primes, PrimeRange, PrimeGenerator, python_impl

@pytest.mark.parametrize("impl, _name", [(PrimeGenerator, "rs"), (python_impl.PrimeGenerator, "py")])
def test_prime_generator(impl, _name):
    p = primes(impl)
    assert next(p) == 2
    assert next(p) == 3
    assert next(p) == 5
    assert next(p) == 7
    *_, p1000 = (next(p) for _ in range(996))
    assert p1000 == 7919
    *_, p10000 = (next(p) for _ in range(9000))
    assert p10000 == 104729
    *_, p100000 = (next(p) for _ in range(90000))
    assert p100000 == 1299709


@pytest.mark.parametrize("impl, _name", [(nth_prime, "rs"), (python_impl.nth_prime, "py")])
def test_nth_prime(impl, _name):
    assert impl(1_000) == 7919
    assert impl(10_000) == 104729
    assert impl(100_000) == 1299709


@pytest.mark.parametrize("impl, _name", [(is_prime, "rs"), (python_impl.is_prime, "py")])
def test_is_prime(impl, _name):
    assert impl(997)
    assert not impl(998)
    assert not impl(1007)
    assert impl(1009)
    assert impl(7919)
    assert impl(104729)
    assert impl(1299709)
    assert not impl(2 ** 30 - 1)
    assert impl(2 ** 31 - 1)

    p = list(PrimeRange(3, 1000))
    for n in range(3, 1000):
        assert not (n in p) ^ is_prime(n)

    p = list(PrimeRange(1000000, 1001000))
    for n in range(1000000, 1001000):
        assert not (n in p) ^ is_prime(n)

@pytest.mark.parametrize("impl, _name", [(prime_factors, "rs"), (python_impl.prime_factors, "py")])
def test_prime_factors(impl, _name):
    with pytest.raises(ValueError):
        impl(0)
    assert impl(1) == []
    assert impl(2) == [2]
    assert impl(3) == [3]
    assert impl(4) == [2, 2]
    assert impl(5) == [5]
    assert impl(6) == [2, 3]

    random.seed(19937)
    numbers = random.choices(range(1000000), k = 100)
    for n in numbers:
        assert reduce(mul, prime_factors(n)) == n

    numbers = random.choices(range(100000000, 101000000), k = 10)
    for n in numbers:
        assert reduce(mul, prime_factors(n)) == n
