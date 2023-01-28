use pyo3::prelude::*;
use pyo3::types::{PyCFunction, PyAny, PyTuple, PyDict};
use std::time::{Instant};
use pyo3::exceptions::PyValueError;


// py::cpp_function exectime(py::function f)
// {
//     return [=](py::args args, const py::kwargs& kwargs) {
//         Timer t;
//         py::object result = f(*args, **kwargs);
//         py::print("elapsed (ms): ", t.elapsed_ms());
//         return result;
//     };
// }


// pub fn exectime(py: &Python, func: PyCFunction) -> PyResult<&PyCFunction> {
//   let timer_func = |args: &PyTuple, kwargs: Option<&PyDict>| {
//     println!("calling func");
//   };

//   let func_py = PyCFunction::new_closure(*py, None, None, timer_func).unwrap();
//   Ok(func_py)
// }


// Based on https://pyo3.rs/v0.18.0/class/call.html?highlight=decorator%20example#example-implementing-a-call-counter
#[pyclass(name = "exectime")]
pub struct ExecTime {
    wraps: Py<PyAny>,
}

#[pymethods]
impl ExecTime {
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
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
    ) -> PyResult<Py<PyAny>> {

        let now = Instant::now();

        let ret = self.wraps.call(py, args, kwargs)?;

        println!("elapsed (ms): {}", now.elapsed().as_millis());

        Ok(ret)
    }
}


#[pyclass(name = "average_exectime")]
pub struct AverageExecTime {
    n: usize
}

#[pyclass]
pub struct AverageExecTimeInner {
  n: usize,
  wraps: Py<PyAny>,
}

#[pymethods]
impl AverageExecTime {
    #[new]
    #[pyo3(signature = (*, n))]
    fn __new__(n: usize) -> Self {
      AverageExecTime {
            n
        }
    }

    fn __call__(
        &self,
        wraps: Py<PyAny>
    ) -> PyResult<AverageExecTimeInner> {
        Ok(AverageExecTimeInner::__new__(self.n, wraps))
    }
}

#[pymethods]
impl AverageExecTimeInner {
    #[new]
    fn __new__(n: usize, wraps: Py<PyAny>) -> Self {
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
    ) -> PyResult<Py<PyAny>> {

        let now = Instant::now();

        match self.n {
          0 => Err(PyValueError::new_err("n must be <=1")),
          _ => {
            for _ in 0..self.n - 1 {
                self.wraps.call(py, args, kwargs)?;
              }
              let result = self.wraps.call(py, args, kwargs)?;

              println!("mean elapsed (ms): {}", now.elapsed().as_millis() as f64 / self.n as f64);

              Ok(result)
          }
        }

        // for _ in 0..self.n - 1 {
        //   self.wraps.call(py, args, kwargs)?;
        // }
        // let result = self.wraps.call(py, args, kwargs)?;

        // println!("elapsed (ms): {} per call, {} calls", now.elapsed().as_millis() as f64 / self.n as f64, self.n);

        // Ok(result)
    }
}
