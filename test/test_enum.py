from enum import Enum

import pyo3_examples as outerscope
from pyo3_examples import RustEnum


class PyEnum(Enum):
    SEVEN = 7
    EIGHT = 8


def test_enum():
    # Rust enums implicitly equate to integers
    assert RustEnum.ONE == 1
    assert RustEnum.TWO == 2
    # python ones don't
    assert PyEnum.SEVEN != 7
    assert PyEnum.EIGHT != 8

def test_enum_scope() -> None:
    # enum values dont pollute outer scope (like C++, unlike C enums)
    # export_values() pollutes outer scope
    assert not hasattr(outerscope, "ONE")
    assert not hasattr(outerscope, "TWO")
    assert not hasattr(outerscope, "THREE")
    assert not hasattr(outerscope, "FIVE")
