import importlib.metadata
from collections.abc import Generator

__version__ = importlib.metadata.version("pyo3-examples")

from _pyo3_examples import *


def primes(cls: type) -> Generator[int, None, None]:
    """Wrapper for generator impls that start at 5 for efficiency"""
    yield 2
    yield 3
    p = cls()
    yield from p
