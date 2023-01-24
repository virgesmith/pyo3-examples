# poetry-rust-integration

Example of a poetry project implemented in rust using maturin and pyo3.

The aim is to replicate the functionality of the C++ equivalent [here](https://github.com/virgesmith/poetry-pybind11-integration).

# steps

```sh
rustup update

# poetry init
# poetry add --group dev maturin pytest

poetry install
```

# build

```sh
maturin develop
```

# test

```sh
pytest
```

# package

```sh
maturin build
```

#  TODO

```poetry build``` doesn't work, use `maturin develop` for now





# see also

https://www.maturin.rs/tutorial.html