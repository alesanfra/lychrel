# Lychrel

[![PyPI](https://img.shields.io/pypi/v/lychrel)](https://pypi.org/project/lychrel/)
[![Documentation](https://readthedocs.org/projects/lychrel/badge/?version=latest)](https://lychrel.readthedocs.io)

Lychrel numbers and other number-theory curiosities for Python, written in
Rust with [PyO3](https://pyo3.rs/). Every function accepts Python integers
of any size.

```console
pip install lychrel
```

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

| Function | What it computes |
| --- | --- |
| `find_lychrel_palindrome` | First palindrome of the [reverse-and-add](https://en.wikipedia.org/wiki/Lychrel_number) process |
| `is_lychrel_candidate` | Whether reverse-and-add fails to reach a palindrome |
| `fibonacci` | Fibonacci numbers |
| `lucas` | Lucas numbers |
| `horadam` | Any recurrence W(n) = p·W(n-1) - q·W(n-2), such as [Pell or Jacobsthal](https://en.wikipedia.org/wiki/Lucas_sequence) |
| `look_and_say` | Next term of the [look-and-say sequence](https://en.wikipedia.org/wiki/Look-and-say_sequence) |
| `kaprekar` | Fixed point of [Kaprekar's routine](https://en.wikipedia.org/wiki/Kaprekar%27s_routine) |
| `collatz` | [Collatz](https://en.wikipedia.org/wiki/Collatz_conjecture) (3n + 1) sequence |

Defaults, limits, and errors are covered in the
[documentation](https://lychrel.readthedocs.io).

## Upgrading to 0.9

`fibonacci` now takes only `number` and returns the Fibonacci numbers. For
other values of `p` and `q`, use `horadam`:

```python
lychrel.fibonacci(n, p, q)  # before 0.9
lychrel.horadam(n, p=p, q=q)  # 0.9 and later
```

See the [changelog](CHANGELOG.md) for everything else.

## Development

You need [uv](https://docs.astral.sh/uv/) and a Rust toolchain.

```console
uv venv -p 3.14
uv sync --frozen
uv run maturin develop --uv
uv run --no-sync pytest
```

See the [development guide](https://lychrel.readthedocs.io/en/latest/development/)
for benchmarks, linting, and releases.

## License

[MIT](LICENSE)
