

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
    println!("destructing a Thing");
  }
}


// TODO - is unsendable a problem?

#[pyclass(unsendable)]
pub struct ManagedThing {
  // deferred init params -> Thing
  initialiser: Box<dyn Fn() -> Thing>,
  resource: Option<Thing>
}


#[pymethods]
impl ManagedThing {
  #[new]
  fn __new__(i: i32, j: i32) -> Self {
    ManagedThing{initialiser: Box::new(move || Thing{i, j}), resource: None}
  }

  fn __call__(&self) -> PyResult<i32> {
    match &self.resource {
      Some(r) => Ok(r.go()),
      _ => Err(PyReferenceError::new_err("cannot access managed resource outside context manager"))
    }
  }

  fn __enter__(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.resource = Some((slf.initialiser)());
    slf
  }

  fn __exit__(mut slf: PyRefMut<'_, Self>, _err_type: Option<&PyType>, _err_msg: Option<&PyString>, _traceback: Option<&PyAny>) -> PyResult<()> {
    slf.resource = None;
    Ok(())
  }
}
