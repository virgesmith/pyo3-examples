import importlib.metadata
from typing import Generator
from _pyo3_examples import *

__version__ = importlib.metadata.version("pyo3-examples")


def primes(cls: type) -> Generator[int, None, None]:
    """Wrapper for generator impls that start at 5 for efficiency"""
    yield 2
    yield 3
    p = cls()
    yield from p
