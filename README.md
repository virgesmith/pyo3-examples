# pyo3-examples

Example of a rust implementation of certain python constructs.

The aims are twofold:

- replicate the functionality of the C++/pybind11 equivalent [here](https://github.com/virgesmith/pybind11-examples), and compare the ease of implementation
- compare performance by implementing the some basic prime number generation and factorisation algorithms in python, C++ and rust.

## build

```sh
pip install -e .[dev]
```

## test

```sh
pytest
```

## compare

Install [pybind11-examples](https://github.com/virgesmith/pybind11-examples) in the same env.

```sh
python performance.py
```

to run performance tests.

## package

```sh
maturin build --release
```

## feature completeness

Compared to [pybind11-examples](https://github.com/virgesmith/pybind11-examples). When I say easier I mean easier **in the end** ;)

- [X] generators. Ever so slightly easier than pybind11 thanks to `PyErr::from_type`.
- [X] `__init_subclass__`. Was actually easier than in C++/pybind11, which requires a workaround for classmethods.
- [X] decorators. Definitely more tricky, but got there in the end thanks to SO.
- [X] context managers. More boilerplate needed - in C++ I can just selectively expose template specialisations to python.
- [ ] constants
- [ ] enums
- [ ] vectorised functions
- [ ] type annotations

## performance comparison

rust initially seemed faster but after some performance improvements of the algorithms (in all languages), overall speed is much improved and C++ has more than caught up. Here's the latest results, times are in milliseconds:

| function      | parameters                           |   rust |   python3.11 |   C++ |
|:--------------|:-------------------------------------|-------:|-------------:|------:|
| nth_prime     | (100000,)                            |      7 |          139 |     6 |
| nth_prime     | (1000000,)                           |    145 |         2115 |    81 |
| prime_factors | (2199023255551,)                     |      7 |         1231 |     7 |
| prime_factors | (10000000000000068,)                 |   1074 |            - |   844 |
| PrimeRange    | (1000000000000000, 1000000000001000) |    722 |            - |   533 |
| is_prime      | (10000000000000061,)                 |   1072 |            - |   855 |


## see also

[https://www.maturin.rs/tutorial.html](https://www.maturin.rs/tutorial.html)

[https://pyo3.rs](https://pyo3.rs)

[https://docs.rs/pyo3/latest/pyo3/](https://docs.rs/pyo3/latest/pyo3/)
