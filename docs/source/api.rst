API Reference
=============

This page contains the complete API reference for all functions in the Lychrel library.

Core Functions
--------------

Lychrel Numbers
~~~~~~~~~~~~~~~

.. autofunction:: lychrel.is_lychrel_candidate

.. autofunction:: lychrel.find_lychrel_palindrome

Fibonacci Sequences
~~~~~~~~~~~~~~~~~~~

.. autofunction:: lychrel.fibonacci

Kaprekar's Routine
~~~~~~~~~~~~~~~~~~

.. autofunction:: lychrel.kaprekar

Look-and-Say Sequence
~~~~~~~~~~~~~~~~~~~~~

.. autofunction:: lychrel.look_and_say

Collatz Conjecture
~~~~~~~~~~~~~~~~~~

.. autofunction:: lychrel.collatz

Pure Python Reference
---------------------

The package includes pure Python implementations in the ``lychrel.py`` module,
primarily used for testing and benchmarking. These implementations follow the
same API as the Rust versions but are significantly slower.

.. automodule:: lychrel.py
   :members:
   :undoc-members:
   :show-inheritance:

Type Information
----------------

The Lychrel library works with Python's standard integer types and handles
arbitrary precision automatically through Rust's ``BigUint`` and ``BigInt`` types.

Supported Input Types
~~~~~~~~~~~~~~~~~~~~~

* ``int`` - Standard Python integers (any size)
* ``bool`` - For parameters like ``is_candidate``

Supported Return Types
~~~~~~~~~~~~~~~~~~~~~~

* ``int`` - For single values (Fibonacci, Kaprekar, etc.)
* ``bool`` - For predicate functions (is_lychrel_candidate)
* ``tuple`` - For functions returning multiple values (find_lychrel_palindrome)
* ``list`` - For sequences (collatz)

Error Handling
--------------

All functions may raise the following exceptions:

``ValueError``
   Raised when input parameters are invalid (e.g., negative numbers where
   only positive are allowed, or when maximum iterations are reached).

``TypeError``
   Raised when input types are incorrect (automatically handled by PyO3).

``OverflowError``
   Rarely raised, only when numbers exceed platform limits (very unlikely
   with BigInt support).

Example error handling:

.. code-block:: python

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

Module Information
------------------

.. py:data:: __version__
   :type: str

   The version string of the installed lychrel package.

   Example:

   .. code-block:: python

      import lychrel
      print(lychrel.__version__)  # e.g., "0.7.2"

Performance Notes
-----------------

All functions in this module are implemented in Rust for maximum performance:

* **No GIL**: Functions release Python's Global Interpreter Lock when possible
* **Native Speed**: Compiled to machine code for optimal execution
* **Memory Efficient**: Direct memory management without garbage collection overhead
* **Arbitrary Precision**: Full support for numbers of any size

For detailed performance information, see :doc:`performance`.

Thread Safety
-------------

All functions in the Lychrel library are **thread-safe** and can be called
from multiple threads simultaneously without issues. The Rust implementation
does not use any shared mutable state.

Example with threading:

.. code-block:: python

   import lychrel
   from concurrent.futures import ThreadPoolExecutor

   def check_number(n):
       return n, lychrel.is_lychrel_candidate(n)

   # Process numbers in parallel
   with ThreadPoolExecutor(max_workers=4) as executor:
       results = list(executor.map(check_number, range(1, 1000)))

   candidates = [n for n, is_cand in results if is_cand]
   print(f"Found {len(candidates)} Lychrel candidates")

Compatibility
-------------

Platform Support
~~~~~~~~~~~~~~~~

* **Linux**: x86_64, aarch64 (ARM64)
* **macOS**: x86_64, aarch64 (Apple Silicon)
* **Windows**: x86_64

Python Version Support
~~~~~~~~~~~~~~~~~~~~~~

* Python 3.7+
* CPython only (PyPy not currently supported)

ABI Compatibility
~~~~~~~~~~~~~~~~~

The package is built with ``abi3`` support, meaning a single wheel works
across multiple Python versions (3.7+), providing better compatibility and
smaller distribution size.

Deprecation Policy
------------------

The Lychrel library follows semantic versioning:

* **Major version** (x.0.0): Breaking API changes
* **Minor version** (0.x.0): New features, backward compatible
* **Patch version** (0.0.x): Bug fixes, backward compatible

Deprecated features will:

1. Be marked as deprecated in documentation
2. Issue a ``DeprecationWarning`` when used
3. Be maintained for at least one minor version
4. Be removed in the next major version

Full Module Index
-----------------

.. autosummary::
   :toctree: generated
   :recursive:

   lychrel
