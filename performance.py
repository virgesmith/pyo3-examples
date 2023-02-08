from __future__ import annotations
from typing import Any, Callable, Generator

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


def run(func: str, *args, generator: bool = False, fast_only=False) -> None:
  for m in fast_modules if fast_only else modules:
    f = getattr(m, func)
    print(m.__name__, func, args)
    if generator:
      _do_gen(f, *args)
    else:
      _do(f, *args)


if __name__ == "__main__":
  # TODO sieve

  run("nth_prime", 100_000)

  run("prime_factors", 2199023255551)
  run("prime_factors", 10000000000000068, fast_only=True)

  m = 10 ** 15
  run("PrimeRange", m, m + 1000, generator=True, fast_only=True)

  n = 10_000_000_000_000_061
  run("is_prime", n, generator=False, fast_only=True)
