import time

import lychrel
import pytest

LYCHREL_NUMBERS = [59, 89, 10_911, 1_186_060_307_891_929_990]


def py_find_palindrome(number):
    next_number = number
    iterations = 0

    while True:
        s = str(next_number)
        r = s[::-1]
        if s == r:
            break

        next_number = next_number + int(r)
        iterations += 1

    return next_number, iterations


@pytest.mark.parametrize("number", LYCHREL_NUMBERS)
def test_lychrel_palindrome(number):
    assert lychrel.lychrel_palindrome(number) == py_find_palindrome(number)[0]


@pytest.mark.parametrize("number", LYCHREL_NUMBERS)
def test_lychrel_iterations(number):
    assert lychrel.lychrel_iterations(number) == py_find_palindrome(number)[1]


@pytest.mark.parametrize("number", LYCHREL_NUMBERS)
def test_lychrel_palindrome_with_iterations(number):
    assert lychrel.lychrel_palindrome_with_iterations(number, 300) == py_find_palindrome(number)


@pytest.mark.parametrize("number, expected_result", [
    (89, False),
    (196, True)
])
def test_is_lychrel_candidate(number, expected_result):
    assert lychrel.is_lychrel_candidate(number) is expected_result


@pytest.mark.parametrize("number", LYCHREL_NUMBERS)
def test_benchmark_implementation(number):
    start_rs = time.time()

    for _ in range(1000):
        _ = lychrel.lychrel_palindrome_with_iterations(number, 300)
    rust_time = time.time() - start_rs

    start_py = time.time()
    for _ in range(1000):
        _ = py_find_palindrome(number)

    py_time = time.time() - start_py

    # Benchmark
    assert py_time > rust_time
