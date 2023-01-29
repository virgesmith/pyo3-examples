# poetry-rust-integration

Example of a poetry project implemented in rust using maturin and pyo3.

The aim is to replicate the functionality of the C++ equivalent [here](https://github.com/virgesmith/poetry-pybind11-integration).

## steps


```sh
# poetry init
# poetry add --group dev maturin pytest
poetry install
```

Or if you're not using poetry just create a virtualenv and

```sh
pip install maturin pytest
```

## build

```sh
maturin develop
```

## test

```sh
pytest
```

## package

```sh
maturin build
```

## feature completeness

- [X] generators
- [X] __init_subclass__
- [X] decorators
- [X] context managers (less generic solution than C++)

## TODO

- ```poetry build``` doesn't work, using `maturin develop` for now
- is it possible to use a function rather than a struct for the (inner) decorator?
- how to capture (deferred) initialisation parameters as a closure and store in a struct?

## see also

https://www.maturin.rs/tutorial.html

https://pyo3.rs

https://docs.rs/pyo3/latest/pyo3/