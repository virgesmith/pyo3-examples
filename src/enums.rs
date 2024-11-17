use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub enum RustEnum {
    ONE = 1,
    TWO = 2,
}
