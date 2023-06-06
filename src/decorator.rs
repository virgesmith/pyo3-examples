use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyDict, PyCFunction};
use std::time::Instant;


#[pyfunction]
pub fn exectime(py: Python, wraps: PyObject) -> PyResult<&PyCFunction> {
    PyCFunction::new_closure(
        py, None, None,
        move |args: &PyTuple, kwargs: Option<&PyDict>| -> PyResult<PyObject> {
            Python::with_gil(|py| {
                let metrics = PyDict::new(py);
                let now = Instant::now();
                let result = wraps.call(py, args, kwargs)?;
                metrics.set_item("elapsed_ms", now.elapsed().as_millis())?;
                Ok((metrics, result).into_py(py))
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
                    let metrics = PyDict::new(py);
                    let mut times = vec![];
                    let mut result: PyObject = py.None();
                    for _ in 0..n {
                        let now = Instant::now();
                        result = wraps.call(py, args, kwargs)?;
                        times.push(now.elapsed().as_millis());
                    }
                    metrics.set_item("max_ms", times.iter().max())?;
                    metrics.set_item("mean_ms", times.iter().sum::<u128>() as f64 / n as f64)?;
                    metrics.set_item("min_ms", times.iter().min())?;
                    Ok((metrics, result).into_py(py))
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
