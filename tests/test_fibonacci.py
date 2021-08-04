import time

import pytest

import lychrel
import lychrel.py


@pytest.mark.parametrize("p", list(range(-10, 10)))
@pytest.mark.parametrize("q", [-3, -2, -1, 0, 1])
def test_fibonacci(p, q):
    assert lychrel.fibonacci(1000, p, q) == lychrel.py.fibonacci(1000, p, q)


def test_benchmark_fibonacci():
    start_rs = time.time()

    for _ in range(10):
        rust_res = lychrel.fibonacci(9999)
    rust_time = time.time() - start_rs

    start_py = time.time()
    for _ in range(10):
        py_res = lychrel.py.fibonacci(9999)

    py_time = time.time() - start_py

    assert py_res == rust_res

    # Benchmark
    assert py_time > rust_time
