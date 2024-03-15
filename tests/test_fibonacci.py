from pyo3_examples import FibGenerator, fib_recursive


def test_fib_recursive() -> None:
    assert fib_recursive(0) == 0
    assert fib_recursive(1) == 1
    assert fib_recursive(2) == 1
    assert fib_recursive(3) == 2
    assert fib_recursive(4) == 3

    for i in range(5, 11):
        assert fib_recursive(i) == fib_recursive(i - 1) + fib_recursive(i - 2)


def test_fibgenerator() -> None:
    fg = FibGenerator()

    assert next(fg) == 0
    series = [next(fg) for _ in range(10)]
    assert series == [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
    assert next(fg) == 89
