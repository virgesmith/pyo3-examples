use pyo3::prelude::*;
// use pyo3::types::{PyType, PyString, PyAny}
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
        if slf.at_end {
            return None;
        }
        let ret = slf.n;
        if slf.n <= 1 {
            slf.at_end = true;
        }
        if slf.n % 2 == 1 {
            slf.n = 3 * slf.n + 1
        } else {
            slf.n /= 2
        }
        Some(ret)
    }

    fn send(mut slf: PyRefMut<'_, Self>, n: u64) -> Option<u64> {
        slf.n = n;
        Self::__next__(slf)
    }

    fn close(mut slf: PyRefMut<'_, Self>) {
        slf.at_end = true;
    }

    // fn throw(_slf: PyRef<'_, Self>, e: PyType, msg: PyString, _traceback: PyAny) {
    //     // TODO...
    //     // - overloaded method
    //     // - how to raise e(msg)
    // }

    #[pyo3(name = "throw")]
    fn throw_default(_slf: PyRef<'_, Self>) -> PyResult<()> {
        Err(PyStopIteration::new_err(""))
    }


}

