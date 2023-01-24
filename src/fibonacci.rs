use pyo3::prelude::*;

#[pyfunction]
pub fn fib_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}


#[pyclass]
pub struct FibGenerator {
    ab: (u64, u64)
}


#[pymethods]
impl FibGenerator {
    #[new]
    fn new() -> Self {
        FibGenerator{ ab: (0, 1) }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u64> {
        let ret = slf.ab.0;
        slf.ab = (slf.ab.1, slf.ab.0 + slf.ab.1);
        Some(ret)
    }
}

