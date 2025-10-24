# Lychrel

[![PyPI version](https://badge.fury.io/py/lychrel.svg)](https://badge.fury.io/py/lychrel)
[![Documentation Status](https://readthedocs.org/projects/lychrel/badge/?version=latest)](https://lychrel.readthedocs.io/en/latest/?badge=latest)

This is a collection of high-performance Python functions implementing famous mathematical problems and algorithms, named after the fascinating [Lychrel numbers](https://en.wikipedia.org/wiki/Lychrel_number).

## Why Lychrel?

**Lychrel** combines the elegance of Python with the performance of Rust, using [PyO3](https://github.com/PyO3/pyo3) to provide:

- 🚀 **High Performance**: Rust implementation makes computations significantly faster than pure Python
- 🔢 **Arbitrary Precision**: Works with numbers of any size using Rust's `num-bigint`
- 🐍 **Pythonic API**: Easy-to-use interface that feels natural in Python
- 📦 **Zero Dependencies**: No external Python dependencies required
- ✅ **Well Tested**: Comprehensive test suite ensuring correctness

User guide and documentation: [lychrel.readthedocs.io](https://lychrel.readthedocs.io)

**Any contribution is welcome!**

## Implemented algorithms

### Lychrel Numbers
Find the first palindrome of the reverse-and-add procedure and determine if a number is a [Lychrel candidate](https://en.wikipedia.org/wiki/Lychrel_number).

A Lychrel number is a natural number that cannot form a palindrome through the iterative process of repeatedly reversing its digits and adding the resulting number to the original.

**Functions:**
- `is_lychrel_candidate(number, max_iterations=None)`: Check if a number is a Lychrel candidate
- `find_lychrel_palindrome(number, max_iterations=None)`: Find the first palindrome and iteration count

### Generalized Fibonacci Sequences
Also known as [Lucas Sequence](https://en.wikipedia.org/wiki/Lucas_sequence), this generalizes the famous Fibonacci sequence.

The sequence is defined by: `F(n) = p*F(n-1) - q*F(n-2)`
- Standard Fibonacci: `p=1, q=-1` → `F(n) = F(n-1) + F(n-2)`
- Lucas numbers: `p=1, q=-1` with different initial values
- Pell numbers: `p=2, q=-1`

**Function:**
- `fibonacci(number, p=None, q=None)`: Compute the nth term of the generalized Fibonacci sequence

### Read Out Loud
Given a number, compute the sequence of digits resulting from reading it out loud, grouping together consecutive occurrences of the same digit.

**Examples:**
- `3211` → `131221` (one 3, one 2, two 1s)
- `1` → `11` (one 1)
- `12` → `1112` (one 1, one 2)

**Function:**
- `look_and_say(number)`: Generate the look-and-say representation

### Kaprekar's Routine
An iterative [algorithm](https://en.wikipedia.org/wiki/Kaprekar%27s_routine) that manipulates digits of a number in a specific base.

With each iteration:
1. Sort the digits in descending order to form the largest possible number
2. Sort the digits in ascending order to form the smallest possible number
3. Subtract the smaller from the larger
4. Repeat until a fixed point (Kaprekar constant) is reached

For 4-digit numbers in base 10, the constant is 6174 (Kaprekar's constant).

**Function:**
- `kaprekar(number, base=None, max_iterations=None)`: Apply Kaprekar's routine

### Collatz Conjecture
Also known as the [3n+1 problem](https://en.wikipedia.org/wiki/Collatz_conjecture), one of mathematics' most famous unsolved problems.

The sequence is defined by:
- If n is even: `n → n/2`
- If n is odd: `n → 3n+1`

The conjecture states that this sequence always reaches 1, regardless of the starting number.

**Function:**
- `collatz(start)`: Generate the Collatz sequence for a given starting number

**Example:** `collatz(5)` returns `[5, 16, 8, 4, 2, 1]`

## Getting started

**Lychrel** is available on [PyPi](https://pypi.org/project/lychrel/), to install it just type on your favourite shell:

```shell
pip install lychrel
```

### Install from source
If you want to install lychrel from source code,
clone this repo and
create a **virtual environment** with python 3.7+ using your favourite tool (conda, virtualenv, etc.),
then follow these steps:

#### 1. Install Rust
If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Install dev requirements
```shell
pip install -r requirements-dev.txt
```

#### 3. Build the package
```shell
maturin develop
```

For optimal performances add the `--release` option:

```shell
maturin develop --release
```

#### 4. Run tests
```shell
pytest
```

To run benchmarks:
```shell
pytest -m benchmark
```

## Examples

### Lychrel Numbers

Check whether a number is a Lychrel candidate:

```python
from lychrel import is_lychrel_candidate, find_lychrel_palindrome

# 196 is a famous Lychrel candidate
assert is_lychrel_candidate(196)

# 197 eventually becomes a palindrome
assert not is_lychrel_candidate(197)

# Find the palindrome and iteration count
palindrome, iterations = find_lychrel_palindrome(89)
print(f"89 becomes palindrome {palindrome} after {iterations} iterations")
# Output: 89 becomes palindrome 8813200023188 after 24 iterations

# Control maximum iterations
is_lychrel_candidate(197, max_iterations=3)  # True (not enough iterations)
is_lychrel_candidate(197, max_iterations=10)  # False (finds palindrome)
```

### Fibonacci Sequences

```python
from lychrel import fibonacci

# Standard Fibonacci sequence
print(fibonacci(10))  # 55

# Lucas sequence (p=1, q=-1, different initial values)
print(fibonacci(10, p=1, q=-1))  # 55

# Pell numbers (p=2, q=-1)
print(fibonacci(10, p=2, q=-1))  # 2378

# Custom parameters
print(fibonacci(10, p=3, q=-2))  # Custom generalized Fibonacci
```

### Kaprekar's Routine

```python
from lychrel import kaprekar

# Classic Kaprekar's constant for 4-digit numbers
result = kaprekar(1234)
print(result)  # 6174

# Different starting numbers converge to the same constant
assert kaprekar(9876) == 6174
assert kaprekar(4680) == 6174

# You can specify a different base
kaprekar(1234, base=10)

# Control maximum iterations
kaprekar(1234, max_iterations=100)
```

### Read Out Loud

```python
from lychrel import look_and_say

print(look_and_say(1))        # 11 (one 1)
print(look_and_say(12))       # 1112 (one 1, one 2)
print(look_and_say(3211))     # 131221 (one 3, one 2, two 1s)
print(look_and_say(2333355))  # 124325 (one 2, three 3s, two 5s)

# Create sequences (Look-and-say sequence)
n = 1
for _ in range(5):
    print(n)
    n = look_and_say(n)
# Output:
# 1
# 11
# 21
# 1211
# 111221
```

### Collatz Conjecture

```python
from lychrel import collatz

# Generate the Collatz sequence
sequence = collatz(5)
print(sequence)  # [5, 16, 8, 4, 2, 1]

# Try with different numbers
print(collatz(27))  # Creates a long sequence (111 steps to reach 1!)

# Find the length of the sequence
print(len(collatz(27)))  # 111

# Visualize the sequence
for i, n in enumerate(collatz(10)):
    print(f"Step {i}: {n}")
```

## Performance

Thanks to the Rust implementation, **Lychrel** provides significant performance improvements over pure Python implementations:

```python
import time
from lychrel import find_lychrel_palindrome
from lychrel.py import find_lychrel_palindrome as find_lychrel_palindrome_py

number = 89

# Rust implementation
start = time.perf_counter()
for _ in range(10000):
    find_lychrel_palindrome(number)
rust_time = time.perf_counter() - start

# Pure Python implementation
start = time.perf_counter()
for _ in range(10000):
    find_lychrel_palindrome_py(number)
python_time = time.perf_counter() - start

print(f"Speedup: {python_time / rust_time:.1f}x faster")
# Typical output: Speedup: 20-50x faster
```

Check out the tests for more examples.

## Requirements

- **Python**: 3.7 or higher
- **Rust**: Only required for building from source

### Supported Platforms

Pre-built wheels are available for:
- **Linux** (x86_64, aarch64)
- **macOS** (x86_64, aarch64/Apple Silicon)
- **Windows** (x86_64)

For other platforms, the package will be built from source automatically if Rust is installed.

## Contributing

Contributions are welcome! Here are some ways you can contribute:

### 🐛 Bug Reports
If you find a bug, please open an issue with:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Python and package version

### 💡 Feature Requests
Have an idea for a new mathematical problem to implement? Open an issue to discuss it!

### 🔧 Pull Requests

1. Fork the repository
2. Create a new branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `pytest`
6. Format your code:
   ```shell
   # Python code
   black .
   isort .

   # Rust code
   cargo fmt
   ```
7. Submit a pull request

### Development Setup

```shell
# Clone the repository
git clone https://github.com/alesanfra/lychrel.git
cd lychrel

# Create a virtual environment
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install development dependencies
pip install -r requirements-dev.txt

# Build the package in development mode
maturin develop --release

# Run tests
pytest

# Run benchmarks
pytest -m benchmark
```

### Code Style

- **Python**: Follow PEP 8, use `black` and `isort` for formatting
- **Rust**: Follow Rust conventions, use `cargo fmt` and `cargo clippy`
- **Documentation**: Add docstrings to all public functions
- **Tests**: Write tests for all new functionality

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [PyO3](https://github.com/PyO3/pyo3) - Rust bindings for Python
- Uses [maturin](https://github.com/PyO3/maturin) for building and packaging
- Inspired by various mathematical curiosities and number theory problems

## References

- [Lychrel Numbers](https://en.wikipedia.org/wiki/Lychrel_number)
- [Lucas Sequences](https://en.wikipedia.org/wiki/Lucas_sequence)
- [Kaprekar's Routine](https://en.wikipedia.org/wiki/Kaprekar%27s_routine)
- [Collatz Conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture)
- [Look-and-say Sequence](https://en.wikipedia.org/wiki/Look-and-say_sequence)

## Related Projects

- [num-bigint](https://github.com/rust-num/num-bigint) - Arbitrary precision integers in Rust
- [PyO3](https://github.com/PyO3/pyo3) - Rust bindings for Python
- [maturin](https://github.com/PyO3/maturin) - Build and publish Rust-based Python packages
