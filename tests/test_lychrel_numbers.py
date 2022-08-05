import time

import pytest

import lychrel
import lychrel.py

LYCHREL_NUMBERS = [59, 89, 10_911, 1_186_060_307_891_929_990]


@pytest.mark.parametrize("number", LYCHREL_NUMBERS)
def test_lychrel_palindrome_with_iterations(number):
    rust_result = lychrel.find_lychrel_palindrome(number)
    python_result = lychrel.py.find_lychrel_palindrome(number)

    assert rust_result == python_result


@pytest.mark.parametrize("number, expected_result", [(89, False), (196, True)])
def test_is_lychrel_candidate(number, expected_result):
    assert lychrel.is_lychrel_candidate(number) is expected_result


@pytest.mark.benchmark
@pytest.mark.parametrize("number", LYCHREL_NUMBERS)
def test_benchmark_lychrel_implementation(number):
    start_rs = time.perf_counter()

    for _ in range(1000):
        _ = lychrel.find_lychrel_palindrome(number)
    rust_time = time.perf_counter() - start_rs

    start_py = time.perf_counter()
    for _ in range(1000):
        _ = lychrel.py.find_lychrel_palindrome(number)

    py_time = time.perf_counter() - start_py

    # Benchmark
    assert py_time > rust_time
