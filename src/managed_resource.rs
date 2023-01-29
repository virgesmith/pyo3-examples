

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

  fn do_the_thing(self: &Self) -> i32
  {
    self.i * self.j
  }
}

impl Drop for Thing {
  fn drop(self: &mut Self) {
    println!("destructing a Thing");
  }
}


// TODO:
// - template
// - deferred init


#[pyclass]
pub struct ManagedThing {
  i: i32,
  j: i32,
  resource: Option<Thing>
  //initialiser:
}


#[pymethods]
impl ManagedThing {
  #[new]
  fn __new__(i: i32, j: i32) -> Self {
    ManagedThing{i, j, resource: None}
  }

  fn __call__(&self) -> PyResult<i32> {
    match &self.resource {
      // PyObject::new ?
      Some(r) => Ok(r.do_the_thing()),
      _ => Err(PyReferenceError::new_err("cannot access managed resource outside context manager"))
    }
  }

  fn __enter__(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
    slf.resource = Some(Thing::new(slf.i, slf.j));
    slf
  }
  // fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
  //   slf
  // }

  //fn (mut slf: PyRefMut<'_, Self>, error_type: Option<&PyType>, error_message: Option<&str>, _traceback: Option<&PyAny>) -> PyResult<()> {
  fn __exit__(mut slf: PyRefMut<'_, Self>, _err_type: Option<&PyType>, _err_msg: Option<&PyString>, _traceback: Option<&PyAny>) -> PyResult<()> {
    slf.resource = None; //.unwrap().drop();
    Ok(())
  }
}





// #[pyclass]
// struct ManagedResource {
//   resource: Option<Box<Thing>>,
//   initialiser: fn(i32, i32) -> Thing
// }


// #[pymethods]
// impl ManagedResource {
//   #[new]
//   // TODO capture i, j in the closure
//   fn __new__(/*i: i32, j: i32*/) -> Self {
//     ManagedResource{resource: None, initialiser: |i, j| { Thing::new(i, j) }}
//   }

//   fn __call__(&self) -> PyResult<PyObject> {
//     match self.resource {
//       Some(r) => Ok(PyInt::new(self.resource.unwrap().do_the_thing())),
//       _ => Err(PyReferenceError("cannot access managed resource outside context manager"))
//     }

//   }

//   fn __enter__(&mut self) -> Self {
//     self.resource = Box::new(self.initialiser(6, 7));
//     self
//   }
//   // fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
//   //   slf
//   // }

//   fn __exit__(&self, _type: PyType, _msg: PyString, _traceback: PyObject) {
//     self.resource.drop()
//   }
// }

