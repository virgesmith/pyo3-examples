use pyo3::prelude::*;

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Clone, Copy, PartialEq)]
pub enum RustEnum {
    One = 1,
    Two = 2,
}
