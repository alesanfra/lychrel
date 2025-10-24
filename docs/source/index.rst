Welcome to Lychrel's documentation!
===================================

**Lychrel** is a high-performance Python library implementing fascinating mathematical problems and algorithms,
named after the mysterious `lychrel numbers <https://en.wikipedia.org/wiki/Lychrel_number>`_.

The library combines the elegance and ease of Python with the performance of Rust, using PyO3 to provide
Python bindings to highly optimized Rust implementations. This approach delivers:

* 🚀 **Exceptional Performance**: 20-50x faster than pure Python implementations
* 🔢 **Arbitrary Precision**: Handle numbers of any size using Rust's BigInt
* 🐍 **Pythonic Interface**: Natural and intuitive API for Python developers
* 📦 **Zero Dependencies**: No external Python packages required
* ✅ **Production Ready**: Comprehensive test coverage and type hints

.. note::

   This project is under active development. Contributions and feedback are always welcome!

What's Inside
-------------

The library currently implements five fascinating mathematical algorithms:

**Lychrel Numbers**
   Explore the reverse-and-add algorithm and discover Lychrel candidates—numbers that may never form palindromes.

**Generalized Fibonacci Sequences**
   Compute terms in Lucas sequences, generalizations of the famous Fibonacci sequence including Pell numbers and more.

**Kaprekar's Routine**
   Apply Kaprekar's mysterious algorithm that leads to the constant 6174 for 4-digit numbers.

**Look-and-Say Sequence**
   Generate Conway's audioactive decay sequence by "reading out loud" the digits of a number.

**Collatz Conjecture**
   Explore the famous 3n+1 problem—one of mathematics' most intriguing unsolved conjectures.

Quick Start
-----------

Installation is simple:

.. code-block:: console

   pip install lychrel

Then start exploring:

.. code-block:: python

   import lychrel

   # Check for Lychrel candidates
   print(lychrel.is_lychrel_candidate(196))  # True

   # Compute Fibonacci numbers
   print(lychrel.fibonacci(10))  # 55

   # Apply Kaprekar's routine
   print(lychrel.kaprekar(1234))  # 6174

   # Generate Collatz sequence
   print(lychrel.collatz(27))  # [27, 82, 41, ..., 2, 1]

Contents
--------

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   usage
   algorithms
   examples

.. toctree::
   :maxdepth: 2
   :caption: API Reference

   api

.. toctree::
   :maxdepth: 1
   :caption: Additional Information

   performance
   contributing

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
