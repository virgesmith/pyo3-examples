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
- [X] context managers. More boilerplate needed - in C++ I can just selectively expose template specialisations to python.

## performance comparison

| function  | parameters | python3.11 | C++ | rust |
|---------------|-----------------------:|-----:|------:|------:|
| nth_prime     |              1 000 000 | 1063 |   145 |    63 |
| is_prime      | 10 000 000 000 000 061 |      |  5728 |  3191 |
| prime_factors |      2 199 023 255 551 | 1253 |   160 |    69 |
| prime_factors | 10 000 000 000 000 068 |      | 43413 | 17357 |
| PrimeRange    |    10^15, 10^15 + 1000 |      |  9613 |  4237 |

times in milliseconds. I am genuinely surprised at how much faster the rust implementation is!

Update: after some performance improvements of the algorithms, speed is much improved and C++ has more than caught up:

| function      | parameters                           |   rust |   python3.11 |   C++ |
|:--------------|:-------------------------------------|-------:|-------------:|------:|
| nth_prime     | (100000,)                            |     67 |         1033 |   136 |
| prime_factors | (2199023255551,)                     |      9 |         1234 |     8 |
| prime_factors | (10000000000000068,)                 |   1075 |          nan |  1071 |
| PrimeRange    | (1000000000000000, 1000000000001000) |    805 |          nan |   523 |
| is_prime      | (10000000000000061,)                 |   1119 |          nan |   849 |
## TODO

- ```poetry build``` doesn't work, using `maturin develop` for now
- how to capture (deferred) initialisation parameters as a closure and store in a struct? with size known at compile time?

## see also

https://www.maturin.rs/tutorial.html

https://pyo3.rs

https://docs.rs/pyo3/latest/pyo3/