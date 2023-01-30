use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyDict, PyCFunction};
use std::time::Instant;


#[pyfunction]
pub fn exectime(py: Python, wraps: PyObject) -> PyResult<&PyCFunction> {
    PyCFunction::new_closure(
        py, None, None,
        move |args: &PyTuple, kwargs: Option<&PyDict>| -> PyResult<PyObject> {
            Python::with_gil(|py| {
                let now = Instant::now();
                let ret = wraps.call(py, args, kwargs);
                println!("elapsed (ms): {}", now.elapsed().as_millis());
                ret
            })
        }
    )
}


// parametrised decorator implemented as function + struct
#[pyfunction]
pub fn average_exectime(py: Python, n: usize) -> PyResult<&PyCFunction> {
    let f = move |args: &PyTuple, _kwargs: Option<&PyDict>| -> PyResult<_> {
        Ok(AverageExecTimeInner::__new__(n, args.get_item(0)?.into()))
    };
    PyCFunction::new_closure(py, None, None, f)
}


#[pyclass]
pub struct AverageExecTimeInner {
    n: usize,
    wraps: PyObject,
}

// Based on https://pyo3.rs/v0.18.0/class/call.html?highlight=decorator%20example#example-implementing-a-call-counter
#[pymethods]
impl AverageExecTimeInner {
    #[new]
    fn __new__(n: usize, wraps: PyObject) -> Self {
        AverageExecTimeInner {
            n,
            wraps
        }
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Option<PyObject>> {
        match self.n {
            0 => Ok(None),
            _ => {
                let now = Instant::now();
                let mut result: Option<PyObject> = None;
                for _ in 0..self.n {
                    result = Some(self.wraps.call(py, args, kwargs)?);
                }
                println!("mean elapsed (ms): {}", now.elapsed().as_millis() as f64 / self.n as f64);
                Ok(result)
            }
        }
    }
}
