"""
        pyo3-examples
        -----------------------
        .. currentmodule:: pyo3_example
        .. autosummary::
           :toctree: _generate
    """

# copied from pybind11-integration and manually edited

from __future__ import annotations

import typing

__all__ = [
    "Collatz",
    "FibGenerator",
    "ManagedThing",
    "PrimeGenerator",
    "PrimeRange",
    "PrimeSieve",
    "Registry",
    "average_exectime",
    "exectime",
    "fib_recursive",
    "is_prime",
    "nth_prime",
    "prime_factors",
    "primes",
]

class Collatz:
    """
    rust implementation of a Collatz sequence generator.
    """

    def __init__(self, n: int) -> None: ...
    def __iter__(self) -> Collatz:
        """
        return iter
        """
    def __next__(self) -> int:
        """
        return next item
        """
    def close(self) -> None:
        """
        generator close
        """
    @typing.overload
    def send(self) -> int:
        """
        generator send (equivalent to next()
        """
    @typing.overload
    def send(self, arg0: int) -> int:
        """
        generator send
        """
    @typing.overload
    def throw(self) -> None:
        """
        generator throw default
        """
    @typing.overload
    def throw(self, type: type, value: str = "", traceback: object = None) -> None:
        """
        generator throw
        """

class FibGenerator:
    """
    rust implementation of a Fibonacci sequence generator.
    """

    def __init__(self) -> None: ...
    def __iter__(self) -> FibGenerator:
        """
        __iter__ dunder
        """
    def __next__(self) -> int:
        """
        __next__ dunder
        """
    pass

class ManagedThing:
    def __call__(self) -> int:
        """
        Here you require at least one lambda to access the wrapped object and perform some operation on/with it.
        The object itself cannot be exposed to python as this will break RAII (you could bind the result of this call to a python variable
        and attempt access outside the context manager, invoking undefined behaviour - the memory will have been released).
        """
    def __enter__(self) -> object:
        """
        Enter context manager.
        """
    def __exit__(
        self, type: type, value: object = "", traceback: object = None
    ) -> None:
        """
        Exit context manager.
        """
    def __init__(self, param1: int, param2: int) -> None: ...
    pass

class PrimeGenerator:
    """
    rust implementation of a prime number generator.
    """

    def __init__(self) -> None: ...
    def __iter__(self) -> PrimeGenerator:
        """
        __iter__ dunder
        """
    def __next__(self) -> int:
        """
        __next__ dunder
        """
    pass

class PrimeRange:
    """
    rust implementation of a prime number generator.
    """

    def __init__(self, start: int, length: int) -> None: ...
    def __iter__(self) -> PrimeRange:
        """
        __iter__ dunder
        """
    def __next__(self) -> int:
        """
        __next__ dunder
        """
    pass

class PrimeSieve:
    """
    rust implementation of a prime number sieve.
    """

    def __init__(self, n: int) -> None: ...
    def __iter__(self) -> PrimeSieve:
        """
        __iter__ dunder
        """
    def __next__(self) -> int:
        """
        __next__ dunder
        """
    pass

class Registry:
    list: dict[type, dict[str, typing.Any]]
    """
    rust implementation base class that accepts __init_subclass__ calls.
    """
    def __init__(self) -> None: ...
    @classmethod
    def __init_subclass__(cls, **kwargs: typing.Any) -> None: ...

def average_exectime(
    *, n: int
) -> typing.Callable[
    ..., typing.Callable[..., tuple[dict[str, float], typing.Any | None]]
]:
    """
    A parameterised decorator that averages execution time for a given number of repeats, implemented in rust
    """

def exectime(
    arg0: typing.Callable,
) -> typing.Callable[..., tuple[dict[str, float], typing.Any | None]]:
    """
    A simple decorator that times execution, implemented in rust
    """

def fib_recursive(n: int) -> int:
    """
    Return nth value in fibonnacci sequence, computed recursively.
    """

def is_prime(n: int) -> bool:
    pass

def nth_prime(n: int) -> int:
    pass

def prime_factors(n: int) -> list[int]:
    pass

def primes(cls: type) -> typing.Generator[int, None, None]:
    pass
