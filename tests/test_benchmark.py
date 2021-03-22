import time

import lychrel


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


def test_benchmark():
    start_rs = time.time()
    for _ in range(1000):
        _ = lychrel.find_palindrome_with_iterations(89)
    rust = time.time() - start_rs

    start_py = time.time()
    for _ in range(1000):
        _ = py_find_palindrome(89)

    py = time.time() - start_py

    assert lychrel.find_palindrome_with_iterations(89) == py_find_palindrome(89)
    assert py == rust
