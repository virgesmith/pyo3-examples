from __future__ import annotations
from typing import Any, Callable, Generator
import pandas as pd


from poetry_rust_integration import primes, python_impl, exectime
import poetry_rust_integration as rust_impl
modules = [rust_impl, python_impl]
fast_modules = [rust_impl]
try:
    import poetry_pybind11_integration as cpp_impl
    modules.append(cpp_impl)
    fast_modules.append(cpp_impl)
except:
    pass


@exectime
def _do(f: Callable, *args: Any) -> Any:
    return f(*args)


@exectime
def _do_gen(f: Generator, *args: Any) -> Any:
    return list(f(*args))


def run(result: pd.DataFrame, func: str, *args, generator: bool = False, fast_only=False) -> None:
  for m in fast_modules if fast_only else modules:
    f = getattr(m, func)
    print(m.__name__, func, args)
    if generator:
      t, _ = _do_gen(f, *args)
    else:
      t, _ = _do(f, *args)
    result.loc[(func, args), m.__name__] = t


if __name__ == "__main__":
  result = pd.DataFrame(columns=["function", "args"] + [m.__name__ for m in modules]).set_index(["function", "args"])

  # TODO sieve

  run(result, "nth_prime", 100_000)

  run(result, "prime_factors", 2199023255551)
  run(result, "prime_factors", 10000000000000068, fast_only=True)

  m = 10 ** 15
  run(result, "PrimeRange", m, m + 1000, generator=True, fast_only=True)

  n = 10_000_000_000_000_061
  run(result, "is_prime", n, generator=False, fast_only=True)
  print(result.reset_index().to_markdown())
