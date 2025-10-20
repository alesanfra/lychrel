Usage
=====

.. _installation:

Installation
------------

To use Lychrel, first install it using pip:

.. code-block:: console

   pip install lychrel

The package includes pre-built wheels for most platforms (Linux, macOS, Windows).
If a wheel is not available for your platform, the package will be built from source
automatically if Rust is installed.

Requirements
~~~~~~~~~~~~

* Python 3.7 or higher
* Rust (only for building from source)

Installing from Source
~~~~~~~~~~~~~~~~~~~~~~

If you want to build from source:

.. code-block:: console

   # Install Rust if not already installed
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Clone the repository
   git clone https://github.com/alesanfra/lychrel.git
   cd lychrel

   # Install development dependencies
   pip install -r requirements-dev.txt

   # Build and install
   maturin develop --release

Check lychrel numbers
---------------------

The Reverse-and-Add Algorithm
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Lychrel numbers are named after an unverified conjecture in recreational mathematics.
To check if a number is a Lychrel candidate, we apply the **reverse-and-add** algorithm:

1. Take a number and reverse its digits
2. Add the reversed number to the original
3. Check if the result is a palindrome
4. If not, repeat the process

A number is considered a **Lychrel candidate** if this process doesn't produce
a palindrome after a certain number of iterations (default: 10000).

Basic Usage
~~~~~~~~~~~

To check whether a number is a lychrel candidate
you can use the ``lychrel.is_lychrel_candidate()`` function:

.. autofunction:: lychrel.is_lychrel_candidate

For example:

>>> import lychrel
>>> lychrel.is_lychrel_candidate(196)
True

The ``lychrel.is_lychrel_candidate()`` function applies iteratively the *reverse-and-add* algorithm until it finds
a palindrome in base-10 representation.

Step-by-Step Example
~~~~~~~~~~~~~~~~~~~~~

For example calling ``lychrel.is_lychrel_candidate(10)`` will result in the following execution:

.. code-block:: console

   iteration 0 -> 10 is not palindrome, reverse the digits and add: 10 + 01 = 11
   iteration 1 -> 11 is palindrome, not a lychrel candidate -> return False

A number is considered a lychrel candidate when the ``lychrel.is_lychrel_candidate()``
cannot find a palindrome after 10000 iterations. You can supply an optional ``max_iterations`` keyword argument
to control the maximum number of iteration to try before declaring a number as a lychrel candidate.

>>> import lychrel
>>> lychrel.is_lychrel_candidate(197)
False
>>> lychrel.is_lychrel_candidate(197, max_iterations=7)
True

Finding the Palindrome
~~~~~~~~~~~~~~~~~~~~~~~

If you want to know both the resulting palindrome and how many iterations it took,
use the ``find_lychrel_palindrome()`` function:

.. autofunction:: lychrel.find_lychrel_palindrome

Example:

>>> import lychrel
>>> palindrome, iterations = lychrel.find_lychrel_palindrome(89)
>>> print(f"Palindrome: {palindrome}, Iterations: {iterations}")
Palindrome: 8813200023188, Iterations: 24

Famous Lychrel Candidates
~~~~~~~~~~~~~~~~~~~~~~~~~~

The most famous Lychrel candidate is **196**. Despite extensive computational searches,
no palindrome has ever been found for this number. Other known candidates include:

* 196
* 295
* 394
* 493
* 592
* 689
* 788
* 887
* 986

All of these numbers (and more) follow similar patterns and are related to 196.
