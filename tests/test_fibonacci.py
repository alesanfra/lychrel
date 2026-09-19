import time

import pytest

import lychrel
import lychrel.py


def test_fibonacci():
    expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
    assert [lychrel.fibonacci(n) for n in range(13)] == expected
    assert lychrel.fibonacci(1000) == lychrel.py.fibonacci(1000)


def test_lucas():
    expected = [2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123]
    assert [lychrel.lucas(n) for n in range(11)] == expected


@pytest.mark.parametrize("p", list(range(-10, 10)))
@pytest.mark.parametrize("q", [-3, -2, -1, 0, 1])
@pytest.mark.parametrize("a, b", [(0, 1), (2, 1), (3, -4), (10**30, 7)])
def test_horadam(a, b, p, q):
    assert lychrel.horadam(300, a, b, p, q) == lychrel.py.horadam(
        300, a, b, p, q
    )


@pytest.mark.parametrize(
    "p, q, expected",
    [
        (2, -1, [0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378]),  # Pell
        (1, -2, [0, 1, 1, 3, 5, 11, 21, 43, 85, 171, 341]),  # Jacobsthal
    ],
)
def test_horadam_first_kind(p, q, expected):
    assert [lychrel.horadam(n, 0, 1, p, q) for n in range(11)] == expected


@pytest.mark.parametrize(
    "p, q, expected",
    [
        (
            2,
            -1,
            [2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786, 6726],
        ),  # Pell-Lucas
        (
            1,
            -2,
            [2, 1, 5, 7, 17, 31, 65, 127, 257, 511, 1025],
        ),  # Jacobsthal-Lucas
    ],
)
def test_horadam_second_kind(p, q, expected):
    assert [lychrel.horadam(n, 2, p, p, q) for n in range(11)] == expected


def test_horadam_defaults():
    assert [lychrel.horadam(n) for n in range(30)] == [
        lychrel.fibonacci(n) for n in range(30)
    ]
    assert [lychrel.horadam(n, a=2) for n in range(30)] == [
        lychrel.lucas(n) for n in range(30)
    ]


@pytest.mark.parametrize("a, b", [(3, 4), (-5, 7), (10**30, -(10**20))])
@pytest.mark.parametrize("p, q", [(1, -1), (2, -1), (3, 2)])
def test_horadam_from_first_kind(a, b, p, q):
    # Any such sequence is W(n) = b * U(n) - q * a * U(n-1).
    for n in range(1, 30):
        u = lychrel.horadam(n, 0, 1, p, q)
        u_prev = lychrel.horadam(n - 1, 0, 1, p, q)
        assert lychrel.horadam(n, a, b, p, q) == b * u - q * a * u_prev


def test_horadam_initial_values():
    assert lychrel.horadam(0, a=3, b=4) == 3
    assert lychrel.horadam(1, a=3, b=4) == 4


@pytest.mark.benchmark
def test_benchmark_fibonacci():
    start_rs = time.perf_counter()

    for _ in range(10):
        rust_res = lychrel.fibonacci(9999)
    rust_time = time.perf_counter() - start_rs

    start_py = time.perf_counter()
    for _ in range(10):
        py_res = lychrel.py.fibonacci(9999)

    py_time = time.perf_counter() - start_py

    assert py_res == rust_res

    # Benchmark
    assert py_time > rust_time
