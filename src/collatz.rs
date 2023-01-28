use pyo3::prelude::*;
use pyo3::types::{PyType, PyAny};
use pyo3::exceptions::PyStopIteration;

#[pyclass]
pub struct Collatz {
    n: u64,
    at_end: bool
}


#[pymethods]
impl Collatz {
    #[new]
    fn new(n: u64) -> Self {
        Collatz{ n, at_end: false }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u64> {
        match slf.at_end {
            true => None,
            _ => {
                let ret = slf.n;
                if slf.n <= 1 {
                    slf.at_end = true;
                }
                match slf.n % 2 {
                    0 => slf.n /= 2,
                    _ => slf.n = 3 * slf.n + 1
                }
                Some(ret)
            }
        }
    }

    fn send(mut slf: PyRefMut<'_, Self>, n: u64) -> Option<u64> {
        slf.n = n;
        Self::__next__(slf)
    }

    fn close(mut slf: PyRefMut<'_, Self>) {
        slf.at_end = true;
    }

    fn throw(_slf: PyRef<'_, Self>, error_type: Option<&PyType>, error_message: Option<&str>, _traceback: Option<&PyAny>) -> PyResult<()> {
        match (error_type, error_message) {
            (Some(e), Some(m)) => Err(PyErr::from_type(e, String::from(m))),
            (Some(e), None) => Err(PyErr::from_type(e, "")),
            _ => Err(PyStopIteration::new_err(""))
        }
    }
}

