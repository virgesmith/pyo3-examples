

use pyo3::prelude::*;
use pyo3::types::{PyString, PyType};
use pyo3::exceptions::PyReferenceError;


struct Thing {
  i: i32,
  j: i32
}


impl Thing {
  fn go(&self) -> i32
  {
    self.i * self.j
  }
}

impl Drop for Thing {
  fn drop(&mut self) {
    eprintln!("destructing a Thing");
  }
}


// Holding the deferred init params (rather than a `Box<dyn Fn>` closure) keeps the
// type `Send`, so no `unsendable` is required.
#[pyclass]
pub struct ManagedThing {
  // deferred init params -> Thing
  i: i32,
  j: i32,
  resource: Option<Thing>
}


#[pymethods]
impl ManagedThing {
  #[new]
  fn __new__(i: i32, j: i32) -> Self {
    ManagedThing{i, j, resource: None}
  }

  fn __call__(&self) -> PyResult<i32> {
    match &self.resource {
      Some(r) => Ok(r.go()),
      _ => Err(PyReferenceError::new_err("cannot access managed resource outside context manager"))
    }
  }

  fn __enter__(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.resource = Some(Thing{ i: slf.i, j: slf.j });
    slf
  }

  fn __exit__(mut slf: PyRefMut<'_, Self>, _err_type: Option<&Bound<'_, PyType>>, _err_msg: Option<&Bound<'_, PyString>>, _traceback: Option<&Bound<'_, PyAny>>) -> PyResult<()> {
    slf.resource = None;
    Ok(())
  }
}
