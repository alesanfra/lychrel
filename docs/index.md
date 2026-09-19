# Lychrel

Lychrel is a small Python library of number-theory curiosities, written in
Rust with [PyO3](https://pyo3.rs/). It works on Python integers of any size.

| Function | What it computes |
| --- | --- |
| [`find_lychrel_palindrome`](reference.md#find_lychrel_palindrome) | First palindrome of the reverse-and-add process |
| [`is_lychrel_candidate`](reference.md#is_lychrel_candidate) | Whether reverse-and-add fails to reach a palindrome |
| [`fibonacci`](reference.md#fibonacci) | Fibonacci numbers |
| [`lucas`](reference.md#lucas) | Lucas numbers |
| [`horadam`](reference.md#horadam) | Any recurrence W(n) = p·W(n-1) - q·W(n-2), such as Pell or Jacobsthal |
| [`look_and_say`](reference.md#look_and_say) | Next term of the look-and-say sequence |
| [`kaprekar`](reference.md#kaprekar) | Fixed point of Kaprekar's routine |
| [`collatz`](reference.md#collatz) | Collatz (3n + 1) sequence |

## Installation

```console
pip install lychrel
```

Wheels are published for Linux, macOS, and Windows, and work on any
CPython from 3.8 on. On other platforms pip builds from source, which
requires a Rust toolchain.

## Quick start

```python
import lychrel

lychrel.find_lychrel_palindrome(89)  # (8813200023188, 24)
lychrel.is_lychrel_candidate(196)    # True
lychrel.fibonacci(10)                # 55
lychrel.lucas(10)                    # 123
lychrel.horadam(10, p=2, q=-1)       # 2378 (Pell numbers)
lychrel.look_and_say(1211)           # 111221
lychrel.kaprekar(3524)               # 6174
lychrel.collatz(5)                   # [5, 16, 8, 4, 2, 1]
```

The [reference](reference.md) describes each function, its defaults, and
the errors it raises.
