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
                let ret = wraps.call(py, args, kwargs)?;
                Ok((now.elapsed().as_millis(), ret).into_py(py))
            })
        }
    )
}


// see https://stackoverflow.com/questions/75308553/implementing-decorators-in-terms-of-closures-with-pyo3/75310631#75310631
#[pyfunction]
pub fn average_exectime(py: Python, n: usize) -> PyResult<&PyCFunction> {
    let f = move |args: &PyTuple, _kwargs: Option<&PyDict>| -> PyResult<Py<PyCFunction>> {
        Python::with_gil(|py| {
            let wraps: PyObject = args.get_item(0)?.into();
            let g = move |args: &PyTuple, kwargs: Option<&PyDict>| -> PyResult<PyObject> {
                Python::with_gil(|py| {
                    let now = Instant::now();
                    let mut result: PyObject = py.None();
                    for _ in 0..n {
                        result = wraps.call(py, args, kwargs)?;
                    }
                    // println!("elapsed (ms): {}", now.elapsed().as_millis());
                    Ok((now.elapsed().as_millis(), result).into_py(py))
                })
            };
            match PyCFunction::new_closure(py, None, None, g) {
                Ok(r) => Ok(r.into()),
                Err(e) => Err(e)
            }
        })
    };
    PyCFunction::new_closure(py, None, None, f)
}
