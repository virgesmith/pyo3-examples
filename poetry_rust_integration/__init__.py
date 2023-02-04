from typing import Generator
from .poetry_rust_integration import *


def primes(cls: type):
    """Wrapper for generator impls that start at 5 for efficiency"""
    yield 2
    yield 3
    p = cls()
    yield from p

