from enum import Enum

import pyo3_examples as outerscope
from pyo3_examples import RustEnum


class PyEnum(Enum):
    Seven = 7
    Eight = 8


def test_enum():
    # Rust enums implicitly equate to integers
    assert RustEnum.One == 1
    assert RustEnum.Two == 2
    # python ones don't
    assert PyEnum.Seven != 7
    assert PyEnum.Eight != 8


def test_enum_scope() -> None:
    # enum values dont pollute outer scope (like C++, unlike C enums)
    # export_values() pollutes outer scope
    assert not hasattr(outerscope, "One")
    assert not hasattr(outerscope, "Two")
