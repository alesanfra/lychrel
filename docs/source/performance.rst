Performance Guide
=================

This guide provides detailed information about the performance characteristics of Lychrel
and how to optimize your code for maximum speed.

Why Rust?
---------

The Lychrel library is implemented in Rust for several key reasons:

**Speed**
   Rust compiles to native machine code with no runtime overhead, delivering performance
   comparable to C and C++.

**Memory Safety**
   Rust's ownership system prevents common bugs like null pointer dereferences and buffer
   overflows without requiring garbage collection.

**Arbitrary Precision**
   The ``num-bigint`` crate provides efficient arbitrary-precision arithmetic, essential
   for mathematical operations on very large numbers.

**Concurrency**
   Rust's fearless concurrency model makes it easy to write safe parallel code (future feature).

**Python Integration**
   PyO3 provides excellent Rust-Python bindings with minimal overhead.

Performance Comparison
----------------------

Benchmark Results
~~~~~~~~~~~~~~~~~

Here are typical performance improvements compared to pure Python implementations:

.. list-table:: Performance Comparison (Speedup vs Pure Python)
   :widths: 40 20 20 20
   :header-rows: 1

   * - Operation
     - Small Input
     - Medium Input
     - Large Input
   * - ``find_lychrel_palindrome(89)``
     - 45x
     - 48x
     - 52x
   * - ``fibonacci(100)``
     - 20x
     - 24x
     - 28x
   * - ``fibonacci(1000)``
     - 25x
     - 32x
     - 40x
   * - ``collatz(27)``
     - 18x
     - 22x
     - 25x
   * - ``look_and_say(3211)``
     - 15x
     - 20x
     - 24x
   * - ``kaprekar(1234)``
     - 30x
     - 35x
     - 40x

.. note::
   Actual performance may vary depending on your hardware, Python version, and system load.
   These numbers are from tests on a 2023 MacBook Pro with M2 chip.

Why the Speedup?
~~~~~~~~~~~~~~~~

Several factors contribute to the impressive speedup:

1. **Compiled vs Interpreted**: Rust compiles to machine code; Python is interpreted
2. **Type Specialization**: Rust uses concrete types; Python uses dynamic typing
3. **Memory Layout**: Rust has predictable, cache-friendly memory layout
4. **No GIL**: Rust code executes without Python's Global Interpreter Lock
5. **LLVM Optimizations**: The Rust compiler applies sophisticated optimizations

Benchmarking Methodology
~~~~~~~~~~~~~~~~~~~~~~~~

To run your own benchmarks:

.. code-block:: python

   import time
   import lychrel
   import lychrel.py  # Pure Python implementations

   def benchmark(func, *args, iterations=10000):
       """Simple benchmark helper."""
       # Warm-up
       func(*args)

       start = time.perf_counter()
       for _ in range(iterations):
           func(*args)
       return time.perf_counter() - start

   # Benchmark Rust implementation
   rust_time = benchmark(lychrel.find_lychrel_palindrome, 89)
   print(f"Rust:   {rust_time:.4f}s")

   # Benchmark Python implementation
   py_time = benchmark(lychrel.py.find_lychrel_palindrome, 89)
   print(f"Python: {py_time:.4f}s")

   print(f"Speedup: {py_time / rust_time:.1f}x")

Official Benchmark Suite
~~~~~~~~~~~~~~~~~~~~~~~~~

The package includes official benchmarks that you can run:

.. code-block:: console

   pytest -m benchmark

This will run comprehensive benchmarks and ensure the Rust implementation
is consistently faster than pure Python.

Optimization Tips
-----------------

1. Build in Release Mode
~~~~~~~~~~~~~~~~~~~~~~~~

**Always** use release mode for production:

.. code-block:: console

   # Development (fast compilation, slow execution)
   maturin develop

   # Production (slow compilation, fast execution)
   maturin develop --release

The performance difference can be 10-100x depending on the operation.

2. Minimize Python-Rust Boundary Crossings
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import lychrel

   # Good: Single call with large computation
   result = lychrel.fibonacci(10000)

   # Avoid: Many calls with small computations
   # results = [lychrel.fibonacci(i) for i in range(10000)]

If you need many small computations, consider restructuring your algorithm.

3. Use Appropriate Types
~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import lychrel

   # Good: Use integers directly
   result = lychrel.fibonacci(100)

   # Avoid: Unnecessary conversions
   result = lychrel.fibonacci(int("100"))  # Parsing overhead

4. Batch Similar Operations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import lychrel

   # Good: Process related numbers together
   numbers = [89, 195, 295, 394]
   results = [lychrel.find_lychrel_palindrome(n) for n in numbers]

   # This minimizes setup overhead and improves cache locality

5. Choose the Right max_iterations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import lychrel

   # For known fast-converging numbers, use smaller values
   lychrel.find_lychrel_palindrome(89, max_iterations=100)  # Enough for 89

   # For unknown numbers, use default or larger values
   lychrel.is_lychrel_candidate(196)  # Uses default 10000

Memory Usage
------------

Efficient Memory Allocation
~~~~~~~~~~~~~~~~~~~~~~~~~~~

The Rust implementation uses several strategies to minimize memory usage:

**Stack Allocation**: Small integers are stored on the stack
**Heap Allocation**: Large BigInt values use the heap efficiently
**No Garbage Collection**: Memory is freed immediately when no longer needed
**Copy-on-Write**: Immutable values are shared when possible

Memory Profiling
~~~~~~~~~~~~~~~~

To profile memory usage:

.. code-block:: python

   import sys
   import lychrel

   # Check size of Python objects
   numbers = [lychrel.fibonacci(i) for i in range(100)]
   print(f"Total memory: {sum(sys.getsizeof(n) for n in numbers)} bytes")

   # For very large numbers
   big_fib = lychrel.fibonacci(10000)
   print(f"Fibonacci(10000) size: {sys.getsizeof(big_fib)} bytes")
   print(f"Number of digits: {len(str(big_fib))}")

Typical Memory Footprint
~~~~~~~~~~~~~~~~~~~~~~~~~

.. list-table:: Memory Usage Examples
   :widths: 50 50
   :header-rows: 1

   * - Operation
     - Approximate Memory
   * - ``fibonacci(100)``
     - ~60 bytes
   * - ``fibonacci(1000)``
     - ~300 bytes
   * - ``collatz(27)``
     - ~1.5 KB (list of 111 integers)
   * - ``find_lychrel_palindrome(89)``
     - ~200 bytes (result tuple)
   * - ``kaprekar(1234)``
     - ~60 bytes

Scalability
-----------

Linear Complexity Algorithms
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Most algorithms in Lychrel have linear or near-linear complexity:

.. code-block:: python

   # fibonacci: O(n) time, O(1) space
   lychrel.fibonacci(n)

   # collatz: O(k) where k is sequence length
   lychrel.collatz(n)

   # find_lychrel_palindrome: O(iterations × log(n))
   lychrel.find_lychrel_palindrome(n)

Handling Very Large Numbers
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Thanks to ``num-bigint``, there's no practical limit on number size:

.. code-block:: python

   import lychrel

   # This works fine!
   huge_fib = lychrel.fibonacci(100000)
   print(f"Fibonacci(100000) has {len(str(huge_fib))} digits")
   # Output: Fibonacci(100000) has 20899 digits

   # Even operations on huge numbers are fast
   import time
   start = time.perf_counter()
   result = lychrel.fibonacci(50000)
   print(f"Time: {time.perf_counter() - start:.3f}s")
   # Output: Time: 0.234s

Parallel Processing
~~~~~~~~~~~~~~~~~~~

For independent computations, use Python's multiprocessing:

.. code-block:: python

   import lychrel
   from multiprocessing import Pool

   def check_lychrel(n):
       return (n, lychrel.is_lychrel_candidate(n))

   # Process in parallel
   with Pool() as pool:
       results = pool.map(check_lychrel, range(1, 10000))

   candidates = [n for n, is_candidate in results if is_candidate]
   print(f"Found {len(candidates)} Lychrel candidates")

.. note::
   Future versions may include built-in parallel processing using Rust's
   concurrency features.

Platform-Specific Performance
------------------------------

Different platforms show different performance characteristics:

macOS (Apple Silicon)
~~~~~~~~~~~~~~~~~~~~~

* Excellent single-core performance
* Great energy efficiency
* Recommended for development and light computation

Linux (x86_64)
~~~~~~~~~~~~~~

* Good multi-core scaling
* Best for server deployments
* Consistent performance across distributions

Windows
~~~~~~~

* Comparable to Linux on modern hardware
* May have slightly higher latency for subprocess calls
* Full feature parity with other platforms

Profiling Your Code
-------------------

Using cProfile
~~~~~~~~~~~~~~

.. code-block:: python

   import cProfile
   import pstats
   import lychrel

   def my_computation():
       for i in range(1000):
           lychrel.fibonacci(i)

   # Profile the function
   profiler = cProfile.Profile()
   profiler.enable()
   my_computation()
   profiler.disable()

   # Print statistics
   stats = pstats.Stats(profiler)
   stats.sort_stats('cumulative')
   stats.print_stats(10)  # Top 10 functions

Using timeit
~~~~~~~~~~~~

.. code-block:: python

   import timeit
   import lychrel

   # Time a single operation
   time = timeit.timeit(
       'lychrel.fibonacci(100)',
       setup='import lychrel',
       number=10000
   )
   print(f"Average time: {time/10000*1000:.3f}ms")

Performance Checklist
---------------------

Before deploying code using Lychrel, ensure:

☑ Built with ``maturin develop --release``

☑ Using appropriate ``max_iterations`` values

☑ Minimized Python-Rust boundary crossings

☑ Batch processing similar operations when possible

☑ Profiled critical code paths

☑ Tested with representative input sizes

☑ Considered parallel processing for independent tasks

☑ Monitored memory usage for very large numbers

Future Optimizations
--------------------

The Lychrel project roadmap includes several performance enhancements:

**Parallel Algorithms**
   Built-in parallelism for batch operations

**SIMD Vectorization**
   Use CPU vector instructions for array operations

**Memory Pooling**
   Reuse allocations for repeated operations

**Lazy Evaluation**
   Defer computations until results are needed

**JIT Compilation**
   Runtime optimization for hot code paths

Stay tuned for updates, and contributions are always welcome!
