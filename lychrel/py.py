"""Pure python implementation of math problems"""


def fibonacci(n, p=1, q=-1):
    """Generalized Fibonacci sequence (aka Lucas sequence)

    The number F(n) is computed with the following  formula: F(n) = p*F(n-1) - q*F(n-2),
    where p and q are two integers. With p==1 and q==-1 we have the standard Fibonacci sequence:
    F(n) = 1*F(n-1) - (-1)*F(n-2) = F(n-1) + F(n-2)

    Args:
        n: position in the sequence of the number to return. E.g. with n==5, the fifth number in the sequence will be returned.
        p: parameter `p` of the formula F(n) = p*F(n-1) - q*F(n-2)
        q: parameter `q` of the formula F(n) = p*F(n-1) - q*F(n-2)
    """
    if n in (0, 1):
        return n

    n1, n2 = 0, 1

    for _ in range(1, n):
        n1, n2 = n2, p * n2 - q * n1

    return n2


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
        n = 3 * n + 1 if n % 2 else n / 2
        yield n
