# API Reference

This page contains the complete API reference for all functions in the Lychrel library.

## Core Functions

### Lychrel Numbers

````markdown
# API Reference

This page contains the complete API reference for all functions in the Lychrel library.

## Core Functions

All functions are implemented in Rust for maximum performance and support arbitrary precision arithmetic.

### Lychrel Numbers

**`lychrel.is_lychrel_candidate(n, max_iterations=100)`**

Check if a number is a Lychrel candidate by testing if it reaches a palindrome.

**Parameters:**

- `n` (int): The number to test
- `max_iterations` (int, optional): Maximum iterations to try (default: 100)

**Returns:** bool - True if no palindrome found within max_iterations, False otherwise

---

**`lychrel.find_lychrel_palindrome(n, max_iterations=100)`**

Find the palindrome by repeatedly adding a number to its reverse.

**Parameters:**

- `n` (int): The starting number
- `max_iterations` (int, optional): Maximum iterations to try (default: 100)

**Returns:** tuple[int, int] - A tuple of (palindrome, iterations_needed)

**Raises:** ValueError if max_iterations is exceeded without finding a palindrome

### Fibonacci Sequences

**`lychrel.fibonacci(n, p=1, q=-1)`**

Calculate the n-th term of a generalized Fibonacci sequence.

**Parameters:**

- `n` (int): The index of the term to calculate (must be >= 0)
- `p` (int, optional): First parameter of the recurrence relation (default: 1)
- `q` (int, optional): Second parameter of the recurrence relation (default: -1)

**Returns:** int - The n-th term of the sequence

**Raises:** ValueError if n is negative

### Kaprekar's Routine

**`lychrel.kaprekar(n, base=10, max_iterations=1000)`**

Apply Kaprekar's routine to a number until reaching a fixed point or cycle.

**Parameters:**

- `n` (int): The starting number
- `base` (int, optional): The numerical base to use (default: 10)
- `max_iterations` (int, optional): Maximum number of iterations (default: 1000)

**Returns:** int - The Kaprekar constant or cycle value reached

**Raises:** ValueError if max_iterations is exceeded or invalid input

### Look-and-Say Sequence

**`lychrel.look_and_say(n)`**

Generate the next term in the Look-and-Say sequence.

**Parameters:**

- `n` (int): The current number to "read out loud"

**Returns:** int - The next number in the sequence

### Collatz Conjecture

**`lychrel.collatz(n)`**

Generate the complete Collatz sequence starting from n until reaching 1.

**Parameters:**

- `n` (int): The starting positive integer (must be > 0)

**Returns:** list[int] - The complete sequence from n to 1 (inclusive)

**Raises:** ValueError if n <= 0

## Pure Python Reference

The package includes pure Python implementations in the `lychrel.py` module, primarily used for testing and benchmarking. These implementations follow the same API as the Rust versions but are significantly slower.

You can import them with:

```python
from lychrel.py import fibonacci, collatz, kaprekar, look_and_say
```

These are useful for:
- Understanding the algorithm implementations
- Educational purposes
- Performance comparisons
- Testing and validation

## Type Information

The Lychrel library works with Python's standard integer types and handles arbitrary precision automatically through Rust's `BigUint` and `BigInt` types.

### Supported Input Types

* `int` - Standard Python integers (any size)
* `bool` - For parameters like `is_candidate`

### Supported Return Types

* `int` - For single values (Fibonacci, Kaprekar, etc.)
* `bool` - For predicate functions (is_lychrel_candidate)
* `tuple` - For functions returning multiple values (find_lychrel_palindrome)
* `list` - For sequences (collatz)

## Error Handling

All functions may raise the following exceptions:

**`ValueError`**
:   Raised when input parameters are invalid (e.g., negative numbers where only positive are allowed, or when maximum iterations are reached).

**`TypeError`**
:   Raised when input types are incorrect (automatically handled by PyO3).

**`OverflowError`**
:   Rarely raised, only when numbers exceed platform limits (very unlikely with BigInt support).

Example error handling:

```python
import lychrel

try:
    result = lychrel.collatz(0)  # Invalid: must be > 0
except ValueError as e:
    print(f"Error: {e}")

try:
    # This might raise if 196 is truly a Lychrel number
    palindrome, iters = lychrel.find_lychrel_palindrome(196, max_iterations=100)
except ValueError as e:
    print(f"Could not find palindrome: {e}")
```

## Module Information

**`__version__`**
:   The version string of the installed lychrel package.

Example:

```python
import lychrel
print(lychrel.__version__)  # e.g., "0.8.0"
```

## Performance Notes

All functions in this module are implemented in Rust for maximum performance:

* **No GIL**: Functions release Python's Global Interpreter Lock when possible
* **Native Speed**: Compiled to machine code for optimal execution
* **Memory Efficient**: Direct memory management without garbage collection overhead
* **Arbitrary Precision**: Full support for numbers of any size

For detailed performance information, see [Performance](performance.md).

## Thread Safety

All functions in the Lychrel library are **thread-safe** and can be called from multiple threads simultaneously without issues. The Rust implementation does not use any shared mutable state.

Example with threading:

```python
import lychrel
from concurrent.futures import ThreadPoolExecutor

def check_number(n):
    return n, lychrel.is_lychrel_candidate(n)

# Process numbers in parallel
with ThreadPoolExecutor(max_workers=4) as executor:
    results = list(executor.map(check_number, range(1, 1000)))

candidates = [n for n, is_cand in results if is_cand]
print(f"Found {len(candidates)} Lychrel candidates")
```

## Compatibility

### Platform Support

* **Linux**: x86_64, aarch64 (ARM64)
* **macOS**: x86_64, aarch64 (Apple Silicon)
* **Windows**: x86_64

### Python Version Support

* Python 3.7+
* CPython only (PyPy not currently supported)

### ABI Compatibility

The package is built with `abi3` support, meaning a single wheel works across multiple Python versions (3.7+), providing better compatibility and smaller distribution size.

## Deprecation Policy

The Lychrel library follows semantic versioning:

* **Major version** (x.0.0): Breaking API changes
* **Minor version** (0.x.0): New features, backward compatible
* **Patch version** (0.0.x): Bug fixes, backward compatible

Deprecated features will:

1. Be marked as deprecated in documentation
2. Issue a `DeprecationWarning` when used
3. Be maintained for at least one minor version
4. Be removed in the next major version

````

## Pure Python Reference

The package includes pure Python implementations in the `lychrel.py` module, primarily used for testing and benchmarking. These implementations follow the same API as the Rust versions but are significantly slower.

::: lychrel.py
    options:
      show_source: false
      members: true

## Type Information

The Lychrel library works with Python's standard integer types and handles arbitrary precision automatically through Rust's `BigUint` and `BigInt` types.

### Supported Input Types

* `int` - Standard Python integers (any size)
* `bool` - For parameters like `is_candidate`

### Supported Return Types

* `int` - For single values (Fibonacci, Kaprekar, etc.)
* `bool` - For predicate functions (is_lychrel_candidate)
* `tuple` - For functions returning multiple values (find_lychrel_palindrome)
* `list` - For sequences (collatz)

## Error Handling

All functions may raise the following exceptions:

**`ValueError`**
:   Raised when input parameters are invalid (e.g., negative numbers where only positive are allowed, or when maximum iterations are reached).

**`TypeError`**
:   Raised when input types are incorrect (automatically handled by PyO3).

**`OverflowError`**
:   Rarely raised, only when numbers exceed platform limits (very unlikely with BigInt support).

Example error handling:

```python
import lychrel

try:
    result = lychrel.collatz(0)  # Invalid: must be > 0
except ValueError as e:
    print(f"Error: {e}")

try:
    # This might raise if 196 is truly a Lychrel number
    palindrome, iters = lychrel.find_lychrel_palindrome(196, max_iterations=100)
except ValueError as e:
    print(f"Could not find palindrome: {e}")
```

## Module Information

**`__version__`**
:   The version string of the installed lychrel package.

Example:

```python
import lychrel
print(lychrel.__version__)  # e.g., "0.8.0"
```

## Performance Notes

All functions in this module are implemented in Rust for maximum performance:

* **No GIL**: Functions release Python's Global Interpreter Lock when possible
* **Native Speed**: Compiled to machine code for optimal execution
* **Memory Efficient**: Direct memory management without garbage collection overhead
* **Arbitrary Precision**: Full support for numbers of any size

For detailed performance information, see [Performance](performance.md).

## Thread Safety

All functions in the Lychrel library are **thread-safe** and can be called from multiple threads simultaneously without issues. The Rust implementation does not use any shared mutable state.

Example with threading:

```python
import lychrel
from concurrent.futures import ThreadPoolExecutor

def check_number(n):
    return n, lychrel.is_lychrel_candidate(n)

# Process numbers in parallel
with ThreadPoolExecutor(max_workers=4) as executor:
    results = list(executor.map(check_number, range(1, 1000)))

candidates = [n for n, is_cand in results if is_cand]
print(f"Found {len(candidates)} Lychrel candidates")
```

## Compatibility

### Platform Support

* **Linux**: x86_64, aarch64 (ARM64)
* **macOS**: x86_64, aarch64 (Apple Silicon)
* **Windows**: x86_64

### Python Version Support

* Python 3.7+
* CPython only (PyPy not currently supported)

### ABI Compatibility

The package is built with `abi3` support, meaning a single wheel works across multiple Python versions (3.7+), providing better compatibility and smaller distribution size.

## Deprecation Policy

The Lychrel library follows semantic versioning:

* **Major version** (x.0.0): Breaking API changes
* **Minor version** (0.x.0): New features, backward compatible
* **Patch version** (0.0.x): Bug fixes, backward compatible

Deprecated features will:

1. Be marked as deprecated in documentation
2. Issue a `DeprecationWarning` when used
3. Be maintained for at least one minor version
4. Be removed in the next major version
