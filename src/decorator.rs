use pyo3::prelude::*;
use pyo3::types::{PyTuple, PyDict, PyCFunction};
use std::time::Instant;


#[pyfunction]
pub fn exectime(py: Python<'_>, wraps: Py<PyAny>) -> PyResult<Bound<'_, PyCFunction>> {
    PyCFunction::new_closure(
        py, None, None,
        move |args: &Bound<'_, PyTuple>, kwargs: Option<&Bound<'_, PyDict>>| -> PyResult<Py<PyAny>> {
            let py = args.py();
            let metrics = PyDict::new(py);
            let now = Instant::now();
            let result = wraps.bind(py).call(args, kwargs)?;
            metrics.set_item("elapsed_ms", now.elapsed().as_millis())?;
            Ok((metrics, result).into_pyobject(py)?.into_any().unbind())
        }
    )
}


// see https://stackoverflow.com/questions/75308553/implementing-decorators-in-terms-of-closures-with-pyo3/75310631#75310631
#[pyfunction]
pub fn average_exectime(py: Python<'_>, n: usize) -> PyResult<Bound<'_, PyCFunction>> {
    let f = move |args: &Bound<'_, PyTuple>, _kwargs: Option<&Bound<'_, PyDict>>| -> PyResult<Py<PyCFunction>> {
        let py = args.py();
        let wraps: Py<PyAny> = args.get_item(0)?.unbind();
        let g = move |args: &Bound<'_, PyTuple>, kwargs: Option<&Bound<'_, PyDict>>| -> PyResult<Py<PyAny>> {
            let py = args.py();
            let metrics = PyDict::new(py);
            let mut times: Vec<u128> = vec![];
            let mut result: Py<PyAny> = py.None();
            for _ in 0..n {
                let now = Instant::now();
                result = wraps.bind(py).call(args, kwargs)?.unbind();
                times.push(now.elapsed().as_micros());
            }
            metrics.set_item("max_ms", *times.iter().max().unwrap() as f64 / 1000.0)?;
            metrics.set_item("mean_ms", times.iter().sum::<u128>() as f64 / n as f64 / 1000.0)?;
            metrics.set_item("min_ms", *times.iter().min().unwrap() as f64 / 1000.0)?;
            Ok((metrics, result).into_pyobject(py)?.into_any().unbind())
        };
        Ok(PyCFunction::new_closure(py, None, None, g)?.unbind())
    };
    PyCFunction::new_closure(py, None, None, f)
}
