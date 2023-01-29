use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyDict, PyCFunction};
use std::time::{Instant};


// TODO get this to work?
// #[pyfunction]
// pub fn exectime2(py: Python, wraps: PyObject) -> PyResult<&PyCFunction> {
//     let f = |args: &PyTuple, kwargs: Option<&PyDict>| -> PyResult<_> {
//         let now = Instant::now();
//         let ret = wraps.call(py, args, kwargs)?;
//         println!("elapsed (ms): {}", now.elapsed().as_millis());
//         Ok(ret)
//     };
//     PyCFunction::new_closure(py, None, None, f)
// }


// Based on https://pyo3.rs/v0.18.0/class/call.html?highlight=decorator%20example#example-implementing-a-call-counter
#[pyclass(name = "exectime")]
pub struct ExecTime {
    wraps: PyObject,
}

#[pymethods]
impl ExecTime {
    #[new]
    fn __new__(wraps: PyObject) -> Self {
        ExecTime {
            wraps
        }
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyResult<PyObject> {
        let now = Instant::now();
        let ret = self.wraps.call(py, args, kwargs)?;
        println!("elapsed (ms): {}", now.elapsed().as_millis());
        Ok(ret)
    }
}

// parameterised decorator implemented as function + struct
#[pyfunction]
pub fn average_exectime(py: Python, n: usize) -> PyResult<&PyCFunction> {
    let f = move |args: &PyTuple, _kwargs: Option<&PyDict>| -> PyResult<_> {
        Ok(AverageExecTimeInner::__new__(n, args.get_item(0)?.into()))
    };
    PyCFunction::new_closure(py, None, None, f)
}


// Original struct-based implementation
// #[pyclass(name = "average_exectime")]
// pub struct AverageExecTime {
//     n: usize
// }


// #[pymethods]
// impl AverageExecTime {
//     #[new]
//     #[pyo3(signature = (*, n))]
//     fn __new__(n: usize) -> Self {
//         AverageExecTime {
//             n
//         }
//     }

//     fn __call__(
//         &self,
//         wraps: PyObject
//     ) -> PyResult<AverageExecTimeInner> {
//         Ok(AverageExecTimeInner::__new__(self.n, wraps))
//     }
// }


#[pyclass]
pub struct AverageExecTimeInner {
    n: usize,
    wraps: PyObject,
}


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
