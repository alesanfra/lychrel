# Lychrel

[![PyPI version](https://badge.fury.io/py/lychrel.svg)](https://badge.fury.io/py/lychrel)

This is a collection of python functions which implements famous mathematical problems, and it is named after
the [lychrel numbers](https://en.wikipedia.org/wiki/Lychrel_number).

Under the hood lychrel is implemented in rust thanks to [PyO3](https://github.com/PyO3/pyo3).

**Any contribution is welcome!**

## Implemented algorithms

- **Lychrel numbers**: find first palindrome of the reverse-and-add procedure and determine if a number is
  a [lychrel candidate](https://en.wikipedia.org/wiki/Lychrel_number).
- **Generalized fibonacci sequences**: also known as [Lucas Sequence](https://en.wikipedia.org/wiki/Lucas_sequence).
- **Read out and loud**: Given a number, this function compute the sequence of digits resulting from reading out loud
  the number, grouping together multiples of the same digit if any. E.g. `read_out_loud(3211) == 131221` because the
  number `3211` is read as "one 3, one 2 and two 1s".
- **Kaprekar's routine**: an iterative [algorithm](https://en.wikipedia.org/wiki/Kaprekar%27s_routine) that, with each
  iteration, takes a natural number in a given number base, creates two new numbers by sorting the digits of its number
  by descending and ascending order, and subtracts the second from the first to yield the natural number for the next
  iteration.
- **Collatz conjecture**: also known as [3n+1 problem](https://en.wikipedia.org/wiki/Collatz_conjecture), implements an
  iterator given a starting number. E.g. `list(collatz(5)) == [5, 16, 8, 4, 2, 1]`.
