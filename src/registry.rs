use pyo3::prelude::*;
use pyo3::types::{PyType, PyDict};
use pyo3::once_cell::GILOnceCell;

static REGISTRY: GILOnceCell<Py<PyDict>> = GILOnceCell::new();

#[pyclass(subclass)]
pub struct Registry{}

#[pymethods]
impl Registry {
    #[allow(non_snake_case)]
    #[classmethod]
    #[pyo3(signature = (**kwargs))]
    fn __init_subclass__(cls: &PyType, kwargs: Option<&PyDict>, py: Python) -> PyResult<()> {
        // just doing
        // REGISTRY.get_or_init(py, || PyDict::new(py).into()).as_ref(py).set_item(cls, kwargs)?;
        // would result in a value of None, rather than {} if kwargs is empty
        // which differs from pybind11's py::kwargs behaviour (but isn't necessarily wrong)
        match kwargs {
            Some(x) => REGISTRY.get_or_init(py, || PyDict::new(py).into()).as_ref(py).set_item(cls, x)?,
            None => REGISTRY.get_or_init(py, || PyDict::new(py).into()).as_ref(py).set_item(cls, PyDict::new(py))?
        }
        Ok(())
    }

    #[classattr]
    fn list(py: Python) -> PyResult<&PyDict> {
        Ok(REGISTRY.get_or_init(py,|| PyDict::new(py).into()).as_ref(py))
    }
}