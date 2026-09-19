# Reference

All functions live in the top-level `lychrel` module. Arguments are Python
`int`s. A negative value where a non-negative one is expected raises
`OverflowError`.

Every function except `collatz` and `look_and_say` releases the GIL while it
computes, so calls from several threads run in parallel:

```python
from concurrent.futures import ThreadPoolExecutor

with ThreadPoolExecutor() as pool:
    results = list(pool.map(lychrel.is_lychrel_candidate, range(1000)))
```

## Lychrel numbers

Reverse-and-add takes a number, reverses its decimal digits, and adds the
two. A [Lychrel number](https://en.wikipedia.org/wiki/Lychrel_number) never
reaches a palindrome this way. None has been proven to exist in base 10;
196 is the smallest candidate.

### find_lychrel_palindrome

```python
find_lychrel_palindrome(number, max_iterations=None) -> tuple[int, int]
```

Returns `(palindrome, iterations)`: the first palindrome reached and the
number of reverse-and-add steps it took. A number that is already a
palindrome returns itself with `0` steps.

`max_iterations` defaults to 10,000 and caps how many values are checked,
the starting number included. If none is a palindrome, the function raises
`ValueError`.

```python
>>> lychrel.find_lychrel_palindrome(89)
(8813200023188, 24)
>>> lychrel.find_lychrel_palindrome(121)
(121, 0)
>>> lychrel.find_lychrel_palindrome(196)
Traceback (most recent call last):
  ...
ValueError: Maximum iterations reached
```

### is_lychrel_candidate

```python
is_lychrel_candidate(number, max_iterations=None) -> bool
```

Returns `True` when `find_lychrel_palindrome` finds no palindrome within
`max_iterations`. A `True` result depends on the limit: 197 reaches a
palindrome after 7 steps, which a limit of 7 does not allow.

```python
>>> lychrel.is_lychrel_candidate(196)
True
>>> lychrel.is_lychrel_candidate(197)
False
>>> lychrel.is_lychrel_candidate(197, max_iterations=7)
True
```

Numbers that reach a palindrome do so quickly, so a candidate costs the full
`max_iterations`. Lower the limit when scanning ranges:

```python
>>> [n for n in range(1000) if lychrel.is_lychrel_candidate(n, max_iterations=1000)]
[196, 295, 394, 493, 592, 689, 691, 788, 790, 879, 887, 978, 986]
```

## Fibonacci, Lucas, and Horadam

### fibonacci

```python
fibonacci(number) -> int
```

Returns the Fibonacci number F(`number`): F(0) = 0, F(1) = 1,
F(n) = F(n-1) + F(n-2). The computation is iterative and takes `number`
steps.

```python
>>> lychrel.fibonacci(10)
55
>>> len(str(lychrel.fibonacci(1000)))
209
```

!!! note "Changed in 0.9"
    `fibonacci` no longer takes `p` and `q`. Replace `fibonacci(n, p, q)`
    with `horadam(n, p=p, q=q)`.

### lucas

```python
lucas(number) -> int
```

Returns the Lucas number L(`number`): L(0) = 2, L(1) = 1,
L(n) = L(n-1) + L(n-2).

```python
>>> lychrel.lucas(10)
123
```

### horadam

```python
horadam(number, a=None, b=None, p=None, q=None) -> int
```

Returns W(`number`) for the [Horadam sequence](https://en.wikipedia.org/wiki/Horadam_sequence)

```
W(0) = a
W(1) = b
W(n) = p * W(n-1) - q * W(n-2)
```

The defaults are `a=0`, `b=1`, `p=1`, `q=-1`, which give the Fibonacci
numbers. `a` and `b` can be integers of any size; `p` and `q` must fit in a
signed 64-bit integer.

With `a=0, b=1` the result is the
[Lucas sequence](https://en.wikipedia.org/wiki/Lucas_sequence) of the first
kind U(p, q). With `a=2, b=p` it is the Lucas sequence of the second kind
V(p, q). Many named sequences are special cases:

| `p` | `q` | U: `a=0, b=1` | V: `a=2, b=p` |
| --- | --- | --- | --- |
| 1 | -1 | Fibonacci: 0, 1, 1, 2, 3, 5, 8 | Lucas: 2, 1, 3, 4, 7, 11, 18 |
| 2 | -1 | Pell: 0, 1, 2, 5, 12, 29, 70 | Pell-Lucas: 2, 2, 6, 14, 34, 82, 198 |
| 1 | -2 | Jacobsthal: 0, 1, 1, 3, 5, 11, 21 | Jacobsthal-Lucas: 2, 1, 5, 7, 17, 31, 65 |
| 3 | 2 | Mersenne (2^n - 1): 0, 1, 3, 7, 15, 31, 63 | 2^n + 1: 2, 3, 5, 9, 17, 33, 65 |

```python
>>> lychrel.horadam(10, p=2, q=-1)  # Pell
2378
>>> lychrel.horadam(5, a=2, b=2, p=2, q=-1)  # Pell-Lucas
82
>>> lychrel.horadam(4, a=3, b=4)
18
```

## look_and_say

```python
look_and_say(number) -> int
```

Reads the digits of `number` aloud and writes down what was said: 1211 is
"one 1, one 2, two 1s", so the result is 111221. Repeating this from 1
produces the [look-and-say sequence](https://en.wikipedia.org/wiki/Look-and-say_sequence).

```python
>>> lychrel.look_and_say(1211)
111221
>>> n = 1
>>> for _ in range(6):
...     print(n)
...     n = lychrel.look_and_say(n)
1
11
21
1211
111221
312211
```

## kaprekar

```python
kaprekar(number, base=None, max_iterations=None) -> int
```

Applies [Kaprekar's routine](https://en.wikipedia.org/wiki/Kaprekar%27s_routine):
sort the digits in descending and ascending order, subtract the smaller
number from the larger, and repeat until the result stops changing. Returns
that fixed point.

```
3524 -> 5432 - 2345 = 3087
3087 -> 8730 -  378 = 8352
8352 -> 8532 - 2358 = 6174
6174 -> 7641 - 1467 = 6174
```

Every value keeps the digit count of `number`, padded with leading zeros:
2111 gives 2111 - 1112 = 999, which continues as 9990 - 0999. So every
four-digit number with at least two distinct digits reaches 6174. Repdigits
such as 1111 reach 0.

`base` defaults to 10. Digits are taken in that base, which must be between 2
and 256. `max_iterations` defaults to 10,000. The function raises
`ValueError` for a base outside that range, or when no fixed point is
reached. Five-digit numbers never reach one: they fall into cycles.

```python
>>> lychrel.kaprekar(3524)
6174
>>> lychrel.kaprekar(2111)
6174
>>> lychrel.kaprekar(123)
495
>>> lychrel.kaprekar(12345)
Traceback (most recent call last):
  ...
ValueError: Maximum iteration reached
```

## collatz

```python
collatz(start) -> list[int]
```

Returns the [Collatz sequence](https://en.wikipedia.org/wiki/Collatz_conjecture)
from `start` down to 1, both included. Each step halves an even number and
maps an odd number `n` to `3n + 1`.

`start` must be positive; zero raises `ValueError`.

```python
>>> lychrel.collatz(5)
[5, 16, 8, 4, 2, 1]
>>> seq = lychrel.collatz(27)
>>> len(seq) - 1, max(seq)
(111, 9232)
```

## Pure-Python versions

`lychrel.py` has plain Python versions of `fibonacci`, `horadam`,
`find_lychrel_palindrome`, and `collatz`, used by the benchmarks. They are
not part of the public API.
