# poetry-rust-integration

Example of a poetry project implemented in rust using maturin and pyo3.

The aims are twofold:

- replicate the functionality of the C++/pybind11 equivalent [here](https://github.com/virgesmith/poetry-pybind11-integration), and compare the ease of implementation
- compare performance by implementing the some basic prime number generation and factorisation algorithms in python, C++ and rust.


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

When I say easier I mean easier **in the end** ;)

- [X] generators. Ever so slightly easier than pybind11 thanks to `PyErr::from_type`.
- [X] `__init_subclass__`. Was actually easier than in C++/pybind11, which requires a workaround for classmethods.
- [X] decorators. Definitely more tricky, but got there in the end thanks to SO.
- [X] context managers. A less generic solution than C++. Not sure how to generalise in rust, or what advantage it will gain.

## performance comparison

| function  | parameters | python | C++ | rust |
|---------------|-----------------------|-------:|------:|-----:|
| nth_prime     | 1 000 000             | 1422   |   147 | 67   |
| prime_factors | 2 199 023 255 551     | 1723   |   204 | 71   |
| PrimeRange    | [10^15, 10^15 + 1000) |        | 10341 | 4355 |

times in milliseconds. I am genuinely surprised at how much faster the rust implementation is!

## TODO

- ```poetry build``` doesn't work, using `maturin develop` for now
- how to capture (deferred) initialisation parameters as a closure and store in a struct? with size known at compile time?

## see also

https://www.maturin.rs/tutorial.html

https://pyo3.rs

https://docs.rs/pyo3/latest/pyo3/