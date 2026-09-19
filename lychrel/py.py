"""Pure python implementation of math problems"""


def horadam(n, a=0, b=1, p=1, q=-1):
    """Term n of W(0) = a, W(1) = b, W(n) = p*W(n-1) - q*W(n-2)."""
    if n == 0:
        return a

    previous, current = a, b

    for _ in range(1, n):
        previous, current = current, p * current - q * previous

    return current


def fibonacci(n):
    """The n-th Fibonacci number."""
    return horadam(n)


def find_lychrel_palindrome(number):
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


def collatz(start):
    if start <= 0:
        raise ValueError("Start number must be > 0")

    yield start

    n = start

    while n != 1:
        n = 3 * n + 1 if n % 2 else n // 2
        yield n
