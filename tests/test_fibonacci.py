import time

import lychrel
import pytest


def py_fibonacci(n, p=1, q=-1):
    if n in (0, 1):
        return n

    n1, n2 = 0, 1

    for _ in range(1, n):
        n1, n2 = n2, p * n2 - q * n1

    return n2


@pytest.mark.parametrize("p", list(range(-10, 10)))
@pytest.mark.parametrize("q", [-3, -2, -1, 0, 1])
def test_fibonacci(p, q):
    assert lychrel.fibonacci(1000, p, q) == py_fibonacci(1000, p, q)


def test_benchmark_fibonacci():
    start_rs = time.time()

    for _ in range(10):
        rust_res = lychrel.fibonacci(9999)
    rust_time = time.time() - start_rs

    start_py = time.time()
    for _ in range(10):
        py_res = py_fibonacci(9999)

    py_time = time.time() - start_py

    assert py_res == rust_res

    # Benchmark
    assert py_time > rust_time
