

use pyo3::prelude::*;
use pyo3::types::{PyString, PyType};
use pyo3::exceptions::PyReferenceError;


struct Thing {
  i: i32,
  j: i32
}

impl Thing {
  fn new(i: i32, j: i32) -> Self {
    println!("constructed a Thing");
    Thing{i, j}
  }

  fn do_the_thing(&self) -> i32
  {
    self.i * self.j
  }
}

impl Drop for Thing {
  fn drop(&mut self) {
    println!("destructing a Thing");
  }
}


// TODO can this be done more generically?
// - template?
// - deferred init? a lambda doesn't seem viable for numerous reasons


#[pyclass]
pub struct ManagedThing {
  // Thing initialisation parameters
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
      Some(r) => Ok(r.do_the_thing()),
      _ => Err(PyReferenceError::new_err("cannot access managed resource outside context manager"))
    }
  }

  fn __enter__(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.resource = Some(Thing::new(slf.i, slf.j));
    slf
  }

  fn __exit__(mut slf: PyRefMut<'_, Self>, _err_type: Option<&PyType>, _err_msg: Option<&PyString>, _traceback: Option<&PyAny>) -> PyResult<()> {
    slf.resource = None;
    Ok(())
  }
}
