import importlib.metadata
from collections.abc import Generator

__version__ = importlib.metadata.version("pyo3-examples")

from _pyo3_examples import (
    Collatz,
    FibGenerator,
    ManagedThing,
    PrimeGenerator,
    PrimeRange,
    PrimeSieve,
    Registry,
    RustEnum,
    average_exectime,
    exectime,
    fib_recursive,
    is_prime,
    nth_prime,
    prime_factors,
)


def primes(cls: type) -> Generator[int, None, None]:
    """Wrapper for generator impls that start at 5 for efficiency"""
    yield 2
    yield 3
    p = cls()
    yield from p


__all__ = [
    "Collatz",
    "FibGenerator",
    "ManagedThing",
    "PrimeGenerator",
    "PrimeRange",
    "PrimeSieve",
    "Registry",
    "RustEnum",
    "average_exectime",
    "exectime",
    "fib_recursive",
    "is_prime",
    "nth_prime",
    "prime_factors",
    "primes",
]
