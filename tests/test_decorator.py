from time import sleep
import pytest
from poetry_rust_integration import exectime, average_exectime


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


def test_simple_cpp_decorator() -> None:
    assert decorated_noargs() is None
    assert decorated_posargs(1, 3.1)
    assert decorated_kwargs(1, flag=True) == 6
    assert decorated_kwargs(5) == 5
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


def test_parameterised_cpp_decorator() -> None:
    assert pdecorated_noargs() is None
    assert pdecorated_kwargs(2) == 3
    with pytest.raises(RuntimeError):
        pthrows()

if __name__ == "__main__":
    test_simple_cpp_decorator()
    test_parameterised_cpp_decorator()
