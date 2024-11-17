from __future__ import annotations

from typing import TYPE_CHECKING, Any

import pandas as pd

import pyo3_examples as rust_impl
from pyo3_examples import exectime, python_impl

if TYPE_CHECKING:
    from collections.abc import Callable

modules = [rust_impl, python_impl]
fast_modules = [rust_impl]
try:
    import pybind11_examples as cpp_impl  # type: ignore[import]

    modules.append(cpp_impl)
    fast_modules.append(cpp_impl)
except ImportError:
    pass


@exectime
def _do(f: Callable, *args: Any) -> Any:
    return f(*args)


@exectime
def _do_gen(f: Callable, *args: Any) -> Any:
    return list(f(*args))


def run(
    result: pd.DataFrame,
    func: str,
    *args: Any,
    generator: bool = False,
    fast_only: bool = False,
) -> None:
    for m in fast_modules if fast_only else modules:
        f = getattr(m, func)
        print(m.__name__, func, args)
        if generator:
            t, _ = _do_gen(f, *args)
        else:
            t, _ = _do(f, *args)
        result.loc[(func, args), m.__name__] = t["elapsed_ms"]


def main():
    result = pd.DataFrame(
        columns=["function", "parameters"] + [m.__name__ for m in modules],
    ).set_index(["function", "parameters"])

    # TODO sieve

    run(result, "nth_prime", 100_000)
    run(result, "nth_prime", 1_000_000)

    run(result, "prime_factors", 2199023255551)
    run(result, "prime_factors", 10000000000000068, fast_only=True)

    m = 10**15
    run(result, "PrimeRange", m, m + 1000, generator=True, fast_only=True)

    n = 10_000_000_000_000_061
    run(result, "is_prime", n, generator=False, fast_only=True)

    colmap = {
        "pyo3_examples": "rust",
        "pyo3_examples.python_impl": "python",
        "pybind11_examples": "C++",
    }

    print(result.rename(columns=colmap).reset_index().to_markdown(index=False))


if __name__ == "__main__":
    main()
