use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq)]
pub enum RustEnum {
    ONE = 1,
    TWO = 2,
}
