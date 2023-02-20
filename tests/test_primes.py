from functools import reduce
from operator import mul
import random
import pytest

from poetry_rust_integration import primes, python_impl
import poetry_rust_integration as rust_impl
fast_modules = [rust_impl]
try:
    import poetry_pybind11_integration as cpp_impl
    fast_modules.append(cpp_impl)
except:
    pass
modules = [python_impl] + fast_modules



@pytest.mark.parametrize("module", fast_modules)
def test_sieve(module):
    p_py = list(python_impl.prime_sieve(1000))
    p = list(module.PrimeSieve(1000))
    assert p_py == p
    for n in range(1000):
        assert (n in p_py) == python_impl.is_prime(n)


@pytest.mark.parametrize("module", modules)
def test_prime_generator(module):
    p = primes(module.PrimeGenerator)
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


@pytest.mark.parametrize("module", modules)
def test_nth_prime(module):
    with pytest.raises(ValueError):
        module.nth_prime(0)
    assert module.nth_prime(1) == 2
    assert module.nth_prime(2) == 3
    assert module.nth_prime(3) == 5
    assert module.nth_prime(4) == 7
    assert module.nth_prime(1_000) == 7919
    assert module.nth_prime(10_000) == 104729
    assert module.nth_prime(100_000) == 1299709


@pytest.mark.parametrize("module", modules)
def test_is_prime(module):
    assert not module.is_prime(0)
    assert not module.is_prime(1)
    assert module.is_prime(2)
    assert module.is_prime(3)
    assert module.is_prime(997)
    assert not module.is_prime(998)
    assert not module.is_prime(1007)
    assert module.is_prime(1009)
    assert module.is_prime(7919)
    assert module.is_prime(104729)
    assert module.is_prime(1299709)
    assert not module.is_prime(2 ** 30 - 1)
    assert module.is_prime(2 ** 31 - 1)

    p = list(module.PrimeRange(3, 1000))
    for n in range(3, 1000):
        assert (n in p) == module.is_prime(n)

    p = list(module.PrimeRange(1000000, 1001000))
    for n in range(1000000, 1001000):
        assert (n in p) == module.is_prime(n)


@pytest.mark.parametrize("module", modules)
def test_prime_factors(module):
    with pytest.raises(ValueError):
        module.prime_factors(0)
    assert module.prime_factors(1) == []
    assert module.prime_factors(2) == [2]
    assert module.prime_factors(3) == [3]
    assert module.prime_factors(4) == [2, 2]
    assert module.prime_factors(5) == [5]
    assert module.prime_factors(6) == [2, 3]

    random.seed(19937)
    numbers = random.choices(range(1000000), k = 100)
    for n in numbers:
        assert reduce(mul, module.prime_factors(n)) == n

    numbers = random.choices(range(100000000, 101000000), k = 10)
    for n in numbers:
        assert reduce(mul, module.prime_factors(n)) == n
