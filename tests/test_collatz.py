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
    assert list(lychrel.collatz(n)) == expected


def test_collatz_zero():
    with pytest.raises(ValueError):
        lychrel.collatz(0)


def test_benchmark_collatz():
    n = 123456789
    start_rs = time.time()
    rust_res = list(lychrel.collatz(n))
    rust_time = time.time() - start_rs

    start_py = time.time()
    py_res = list(lychrel.py.collatz(n))
    py_time = time.time() - start_py

    assert py_res == rust_res

    # Benchmark
    assert py_time > rust_time
