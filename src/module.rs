use pyo3::prelude::*;

use crate::fibonacci::*;
use crate::collatz::Collatz;
use crate::registry::Registry;
use crate::decorator::{exectime, average_exectime};
use crate::managed_resource::ManagedThing;
use crate::primes::{is_prime_py, nth_prime, prime_factors, PrimeGenerator, PrimeRange};

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn poetry_rust_integration(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib_recursive, m)?)?;
    m.add_function(wrap_pyfunction!(exectime, m)?)?;
    m.add_function(wrap_pyfunction!(average_exectime, m)?)?;

    m.add_class::<FibGenerator>()?;
    m.add_class::<Collatz>()?;
    m.add_class::<Registry>()?;
    m.add_class::<ManagedThing>()?;

    m.add_class::<PrimeGenerator>()?;
    m.add_class::<PrimeRange>()?;

    m.add_function(wrap_pyfunction!(is_prime_py, m)?)?;
    m.add_function(wrap_pyfunction!(nth_prime, m)?)?;
    m.add_function(wrap_pyfunction!(prime_factors, m)?)?;

    Ok(())
}