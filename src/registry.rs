use pyo3::prelude::*;
use pyo3::types::{PyType, PyDict};
use pyo3::sync::PyOnceLock;

static REGISTRY: PyOnceLock<Py<PyDict>> = PyOnceLock::new();

#[pyclass(subclass)]
pub struct Registry{}

#[pymethods]
impl Registry {
    #[allow(non_snake_case)]
    #[classmethod]
    #[pyo3(signature = (**kwargs))]
    fn __init_subclass__(cls: &Bound<'_, PyType>, kwargs: Option<&Bound<'_, PyDict>>, py: Python) -> PyResult<()> {
        // just doing
        // REGISTRY.get_or_init(py, || PyDict::new(py).into()).bind(py).set_item(cls, kwargs)?;
        // would result in a value of None, rather than {} if kwargs is empty
        // which differs from pybind11's py::kwargs behaviour (but isn't necessarily wrong)
        match kwargs {
            Some(x) => REGISTRY.get_or_init(py, || PyDict::new(py).into()).bind(py).set_item(cls, x)?,
            None => REGISTRY.get_or_init(py, || PyDict::new(py).into()).bind(py).set_item(cls, PyDict::new(py))?
        }
        Ok(())
    }

    #[classattr]
    fn list(py: Python<'_>) -> Bound<'_, PyDict> {
        REGISTRY.get_or_init(py, || PyDict::new(py).into()).bind(py).clone()
    }
}