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
def test_find_palindrome(number):
    assert lychrel.find_palindrome(number) == py_find_palindrome(number)[0]


@pytest.mark.parametrize("number", LYCHREL_NUMBERS)
def test_find_palindrome_with_iterations(number):
    start_rs = time.time()
    for _ in range(1000):
        _ = lychrel.find_palindrome_with_iterations(number)
    rust_time = time.time() - start_rs

    start_py = time.time()
    for _ in range(1000):
        _ = py_find_palindrome(number)

    py_time = time.time() - start_py

    assert lychrel.find_palindrome_with_iterations(89) == py_find_palindrome(89)
    assert py_time > rust_time
