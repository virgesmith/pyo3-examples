from time import sleep

import pytest

from pyo3_examples import average_exectime, exectime


@exectime
def decorated_noargs() -> None:
    sleep(0.01)
    print("in decorated python function")


@exectime
def decorated_posargs(x: int, y: float) -> bool:
    sleep(0.02)
    print(f"decorated_posargs: {x} {y}")
    return True


@exectime
def decorated_kwargs(z: float, *, flag: bool = False) -> int:
    sleep(0.03)
    print(f"decorated_kwargs: {z} {flag}")
    return int(z) + (5 if flag else 0)


@exectime
def throws() -> None:
    raise RuntimeError()


def test_simple_rust_decorator() -> None:
    metric, result = decorated_noargs()
    assert isinstance(metric, dict) and "elapsed_ms" in metric
    assert result is None

    assert decorated_posargs(1, 3.1)[1]
    assert decorated_kwargs(1, flag=True)[1] == 6
    assert decorated_kwargs(5)[1] == 5
    with pytest.raises(RuntimeError):
        pthrows()


@average_exectime(n=4)
def pdecorated_noargs() -> None:
    sleep(0.002)
    print("in parameterised decorated python function")


@average_exectime(n=3)
def pdecorated_kwargs(z: float, *, flag: bool = False) -> int:
    sleep(0.003)
    print("in pdecorated_kwargs", z, flag)
    return 3


@average_exectime(n=1)
def pthrows() -> None:
    raise RuntimeError()


def test_parameterised_rust_decorator() -> None:
    metric, result = pdecorated_noargs()
    assert isinstance(metric, dict) and all(
        k in metric.keys() for k in ("max_ms", "mean_ms", "min_ms")
    )
    assert result is None
    assert pdecorated_kwargs(2)[1] == 3
    with pytest.raises(RuntimeError):
        pthrows()


if __name__ == "__main__":
    test_simple_rust_decorator()
    test_parameterised_rust_decorator()
