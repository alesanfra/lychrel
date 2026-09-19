import time

import pytest

import lychrel
import lychrel.py


@pytest.mark.parametrize(
    "n, expected",
    [
        (5, [5, 16, 8, 4, 2, 1]),
    ],
)
def test_collatz(n, expected):
    assert lychrel.collatz(n) == expected


def test_collatz_zero():
    with pytest.raises(ValueError):
        lychrel.collatz(0)


@pytest.mark.parametrize("n", [2**127, 2**128 - 1, 2**300 + 1])
def test_collatz_big_numbers(n):
    assert lychrel.collatz(n) == list(lychrel.py.collatz(n))


@pytest.mark.benchmark
def test_benchmark_collatz():
    n = int(123e20)
    start_rs = time.perf_counter()
    rust_res = lychrel.collatz(n)
    rust_time = time.perf_counter() - start_rs

    start_py = time.perf_counter()
    py_res = list(lychrel.py.collatz(n))
    py_time = time.perf_counter() - start_py

    assert py_res == rust_res

    # Benchmark
    assert py_time > rust_time
