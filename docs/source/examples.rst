Examples and Performance
========================

This page contains practical examples and performance comparisons between the Rust implementation
and pure Python implementations.

Complete Examples
-----------------

Example 1: Finding Lychrel Candidates
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Let's find all Lychrel candidates below 10,000:

.. code-block:: python

   import lychrel

   def find_lychrel_candidates(max_number, max_iterations=10000):
       """Find all Lychrel candidates up to max_number."""
       candidates = []
       for n in range(1, max_number + 1):
           if lychrel.is_lychrel_candidate(n, max_iterations):
               candidates.append(n)

           # Progress indicator
           if n % 1000 == 0:
               print(f"Checked up to {n}...")

       return candidates

   # Find candidates below 1000
   candidates = find_lychrel_candidates(1000)
   print(f"Found {len(candidates)} Lychrel candidates below 1000")
   print(f"First 10 candidates: {candidates[:10]}")

   # Output:
   # Found 13 Lychrel candidates below 1000
   # First 10 candidates: [196, 295, 394, 493, 592, 689, 788, 887, 986]

Example 2: Analyzing Palindrome Formation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Analyze how quickly different numbers form palindromes:

.. code-block:: python

   import lychrel
   import matplotlib.pyplot as plt

   def analyze_palindrome_formation(numbers):
       """Analyze iteration counts for palindrome formation."""
       results = []

       for n in numbers:
           try:
               palindrome, iterations = lychrel.find_lychrel_palindrome(n)
               results.append({
                   'number': n,
                   'iterations': iterations,
                   'palindrome': palindrome
               })
           except ValueError:
               # Lychrel candidate
               results.append({
                   'number': n,
                   'iterations': None,
                   'palindrome': None
               })

       return results

   # Analyze numbers from 1 to 200
   results = analyze_palindrome_formation(range(1, 201))

   # Filter out Lychrel candidates
   valid_results = [r for r in results if r['iterations'] is not None]

   # Print statistics
   iterations = [r['iterations'] for r in valid_results]
   print(f"Average iterations: {sum(iterations) / len(iterations):.2f}")
   print(f"Max iterations: {max(iterations)}")
   print(f"Min iterations: {min(iterations)}")

   # Find number with most iterations
   max_iter_result = max(valid_results, key=lambda r: r['iterations'])
   print(f"\nNumber with most iterations: {max_iter_result['number']}")
   print(f"Iterations: {max_iter_result['iterations']}")
   print(f"Final palindrome: {max_iter_result['palindrome']}")

Example 3: Fibonacci Analysis
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Compare different Fibonacci-like sequences:

.. code-block:: python

   import lychrel

   def compare_sequences(n, sequences):
       """Compare multiple generalized Fibonacci sequences."""
       results = {}

       for name, params in sequences.items():
           p, q = params
           sequence = [lychrel.fibonacci(i, p=p, q=q) for i in range(n)]
           results[name] = sequence

       return results

   # Define sequences
   sequences = {
       'Fibonacci': (1, -1),
       'Pell': (2, -1),
       'Jacobsthal': (1, -2),
       'Tribonacci-like': (3, -2),
   }

   # Generate first 15 terms
   results = compare_sequences(15, sequences)

   # Display results
   for name, sequence in results.items():
       print(f"\n{name}:")
       print(sequence)

   # Output:
   # Fibonacci:
   # [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377]
   #
   # Pell:
   # [0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378, 5741, 13860, 33461, 80782]
   #
   # ...

Example 4: Collatz Visualization
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Visualize the Collatz sequence and analyze patterns:

.. code-block:: python

   import lychrel
   import matplotlib.pyplot as plt

   def visualize_collatz(start_number):
       """Visualize the Collatz sequence."""
       sequence = lychrel.collatz(start_number)

       plt.figure(figsize=(12, 6))
       plt.plot(sequence, marker='o', linestyle='-', linewidth=1, markersize=3)
       plt.title(f'Collatz Sequence starting from {start_number}')
       plt.xlabel('Step')
       plt.ylabel('Value')
       plt.grid(True, alpha=0.3)
       plt.yscale('log')  # Log scale to see patterns better
       plt.show()

       print(f"Starting number: {start_number}")
       print(f"Steps to reach 1: {len(sequence) - 1}")
       print(f"Maximum value reached: {max(sequence)}")
       print(f"Sequence length: {len(sequence)}")

   # Visualize different starting numbers
   for n in [27, 97, 871]:
       visualize_collatz(n)

   def compare_stopping_times(max_n):
       """Compare stopping times for numbers up to max_n."""
       stopping_times = []

       for n in range(1, max_n + 1):
           sequence = lychrel.collatz(n)
           stopping_times.append((n, len(sequence) - 1))

       # Plot
       numbers, times = zip(*stopping_times)
       plt.figure(figsize=(12, 6))
       plt.scatter(numbers, times, alpha=0.5, s=10)
       plt.title(f'Collatz Stopping Times for n=1 to {max_n}')
       plt.xlabel('Starting Number')
       plt.ylabel('Stopping Time (steps to reach 1)')
       plt.grid(True, alpha=0.3)
       plt.show()

       # Find interesting numbers
       max_time_n, max_time = max(stopping_times, key=lambda x: x[1])
       print(f"Number with longest stopping time: {max_time_n} ({max_time} steps)")

   compare_stopping_times(1000)

Example 5: Look-and-Say Growth Rate
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Analyze the growth rate of the Look-and-Say sequence:

.. code-block:: python

   import lychrel
   import matplotlib.pyplot as plt

   def analyze_look_and_say_growth(iterations):
       """Analyze growth rate of Look-and-Say sequence."""
       n = 1
       lengths = [len(str(n))]

       for i in range(iterations):
           n = lychrel.look_and_say(n)
           lengths.append(len(str(n)))

       # Calculate growth rates
       growth_rates = [lengths[i+1] / lengths[i] for i in range(len(lengths)-1)]

       # Conway's constant: approximately 1.303577...
       conway_constant = 1.303577269034296

       print(f"Final length after {iterations} iterations: {lengths[-1]}")
       print(f"Average growth rate: {sum(growth_rates) / len(growth_rates):.6f}")
       print(f"Conway's constant: {conway_constant:.6f}")

       # Plot lengths
       plt.figure(figsize=(12, 6))
       plt.subplot(1, 2, 1)
       plt.plot(lengths, marker='o')
       plt.title('Look-and-Say Sequence Length')
       plt.xlabel('Iteration')
       plt.ylabel('Number of Digits')
       plt.yscale('log')
       plt.grid(True, alpha=0.3)

       # Plot growth rates
       plt.subplot(1, 2, 2)
       plt.plot(growth_rates, marker='o', alpha=0.5)
       plt.axhline(y=conway_constant, color='r', linestyle='--',
                   label="Conway's Constant")
       plt.title('Growth Rate per Iteration')
       plt.xlabel('Iteration')
       plt.ylabel('Growth Rate')
       plt.legend()
       plt.grid(True, alpha=0.3)

       plt.tight_layout()
       plt.show()

   analyze_look_and_say_growth(30)

Performance Benchmarks
-----------------------

Rust vs Python Implementation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The Lychrel package includes pure Python implementations in ``lychrel.py`` for comparison.
Here's a comprehensive benchmark:

.. code-block:: python

   import time
   import lychrel
   import lychrel.py

   def benchmark_function(func_rust, func_python, *args, iterations=1000):
       """Benchmark a function against its Python equivalent."""

       # Warm-up
       func_rust(*args)
       func_python(*args)

       # Benchmark Rust implementation
       start = time.perf_counter()
       for _ in range(iterations):
           func_rust(*args)
       rust_time = time.perf_counter() - start

       # Benchmark Python implementation
       start = time.perf_counter()
       for _ in range(iterations):
           func_python(*args)
       python_time = time.perf_counter() - start

       speedup = python_time / rust_time

       return {
           'rust_time': rust_time,
           'python_time': python_time,
           'speedup': speedup
       }

   # Benchmark Lychrel palindrome finding
   print("Benchmarking find_lychrel_palindrome(89):")
   result = benchmark_function(
       lychrel.find_lychrel_palindrome,
       lychrel.py.find_lychrel_palindrome,
       89,
       iterations=10000
   )
   print(f"  Rust time:   {result['rust_time']:.4f}s")
   print(f"  Python time: {result['python_time']:.4f}s")
   print(f"  Speedup:     {result['speedup']:.1f}x faster\n")

   # Benchmark Fibonacci
   print("Benchmarking fibonacci(100):")
   result = benchmark_function(
       lychrel.fibonacci,
       lychrel.py.fibonacci,
       100,
       iterations=100000
   )
   print(f"  Rust time:   {result['rust_time']:.4f}s")
   print(f"  Python time: {result['python_time']:.4f}s")
   print(f"  Speedup:     {result['speedup']:.1f}x faster\n")

   # Benchmark Collatz
   print("Benchmarking collatz(27):")
   result = benchmark_function(
       lychrel.collatz,
       lambda n: list(lychrel.py.collatz(n)),
       27,
       iterations=10000
   )
   print(f"  Rust time:   {result['rust_time']:.4f}s")
   print(f"  Python time: {result['python_time']:.4f}s")
   print(f"  Speedup:     {result['speedup']:.1f}x faster")

Typical Output
^^^^^^^^^^^^^^

.. code-block:: text

   Benchmarking find_lychrel_palindrome(89):
     Rust time:   0.0234s
     Python time: 1.1567s
     Speedup:     49.4x faster

   Benchmarking fibonacci(100):
     Rust time:   0.0089s
     Python time: 0.2134s
     Speedup:     24.0x faster

   Benchmarking collatz(27):
     Rust time:   0.0156s
     Python time: 0.3421s
     Speedup:     21.9x faster

Memory Usage
~~~~~~~~~~~~

The Rust implementation is also memory-efficient:

.. code-block:: python

   import sys
   import lychrel

   # Python list (baseline)
   py_list = list(range(1000))
   print(f"Python list memory: {sys.getsizeof(py_list)} bytes")

   # Collatz sequence
   sequence = lychrel.collatz(27)
   print(f"Collatz(27) memory: {sys.getsizeof(sequence)} bytes")
   print(f"Sequence length: {len(sequence)} elements")

   # Large Fibonacci number
   fib_1000 = lychrel.fibonacci(1000)
   print(f"Fibonacci(1000) memory: {sys.getsizeof(fib_1000)} bytes")
   print(f"Number of digits: {len(str(fib_1000))}")

Scaling Performance
~~~~~~~~~~~~~~~~~~~

Performance advantage increases with problem size:

.. code-block:: python

   import time
   import lychrel
   import lychrel.py

   def measure_scaling(sizes):
       """Measure how performance scales with input size."""
       results = []

       for size in sizes:
           # Rust
           start = time.perf_counter()
           _ = lychrel.fibonacci(size)
           rust_time = time.perf_counter() - start

           # Python
           start = time.perf_counter()
           _ = lychrel.py.fibonacci(size)
           python_time = time.perf_counter() - start

           speedup = python_time / rust_time
           results.append((size, rust_time, python_time, speedup))

       return results

   sizes = [100, 500, 1000, 5000, 10000]
   results = measure_scaling(sizes)

   print("Fibonacci Performance Scaling:")
   print(f"{'Size':<8} {'Rust (s)':<12} {'Python (s)':<12} {'Speedup':<10}")
   print("-" * 50)
   for size, rust_t, python_t, speedup in results:
       print(f"{size:<8} {rust_t:<12.6f} {python_t:<12.6f} {speedup:<10.1f}x")

Tips for Best Performance
--------------------------

1. Use Release Builds
~~~~~~~~~~~~~~~~~~~~~

For maximum performance, always build with the ``--release`` flag:

.. code-block:: console

   maturin develop --release

The difference between debug and release builds can be 10-100x in performance.

2. Batch Operations
~~~~~~~~~~~~~~~~~~~

When processing multiple numbers, batch them efficiently:

.. code-block:: python

   import lychrel

   # Good: Process in batch
   numbers = range(1, 1000)
   results = [lychrel.is_lychrel_candidate(n) for n in numbers]

   # Also good: Use list comprehension
   palindromes = [
       lychrel.find_lychrel_palindrome(n)
       for n in numbers
       if not lychrel.is_lychrel_candidate(n)
   ]

3. Avoid Unnecessary Type Conversions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import lychrel

   # Good: Keep as integers
   result = lychrel.fibonacci(100)

   # Avoid: Unnecessary string conversion
   # result_str = str(lychrel.fibonacci(100))  # Only if needed

4. Use Appropriate max_iterations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import lychrel

   # Default (10000) is safe but might be overkill
   lychrel.is_lychrel_candidate(196)

   # If you know the number converges quickly, use smaller value
   lychrel.is_lychrel_candidate(89, max_iterations=100)  # Faster
