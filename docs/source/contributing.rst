Contributing
============

Thank you for your interest in contributing to Lychrel! This document provides
guidelines and information for contributors.

Ways to Contribute
------------------

There are many ways to contribute to Lychrel:

🐛 **Report Bugs**
   Found a bug? Open an issue with reproduction steps

💡 **Suggest Features**
   Have an idea for a new algorithm? Let's discuss it!

📖 **Improve Documentation**
   Help make the docs clearer and more comprehensive

🧪 **Add Tests**
   Increase test coverage and add edge cases

⚡ **Optimize Code**
   Improve performance or memory usage

🔧 **Fix Issues**
   Pick up an open issue and submit a PR

Getting Started
---------------

Setting Up Development Environment
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. **Fork and Clone**

.. code-block:: console

   git clone https://github.com/YOUR_USERNAME/lychrel.git
   cd lychrel

2. **Install Rust**

If you don't have Rust installed:

.. code-block:: console

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

3. **Create Virtual Environment**

.. code-block:: console

   python -m venv .venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate

4. **Install Development Dependencies**

.. code-block:: console

   pip install -r requirements-dev.txt

5. **Build in Development Mode**

.. code-block:: console

   maturin develop

6. **Run Tests**

.. code-block:: console

   pytest

Development Workflow
--------------------

1. Create a Branch
~~~~~~~~~~~~~~~~~~

Create a descriptive branch name:

.. code-block:: console

   git checkout -b feature/add-new-algorithm
   git checkout -b fix/issue-123
   git checkout -b docs/improve-examples

2. Make Changes
~~~~~~~~~~~~~~~

**For Python Code:**

.. code-block:: python

   # lychrel/py.py or tests/

   def new_function(n):
       """Add comprehensive docstring.

       Args:
           n: Description of parameter

       Returns:
           Description of return value

       Raises:
           ValueError: When and why this is raised
       """
       pass

**For Rust Code:**

.. code-block:: rust

   // src/lib.rs

   /// Comprehensive documentation comment
   ///
   /// # Arguments
   ///
   /// * `number` - Description of parameter
   ///
   /// # Returns
   ///
   /// Description of return value
   ///
   /// # Errors
   ///
   /// When and why errors occur
   #[pyfunction]
   #[pyo3(signature = (number, param=None))]
   fn new_function(number: BigUint, param: Option<usize>) -> PyResult<BigUint> {
       // Implementation
       Ok(number)
   }

3. Add Tests
~~~~~~~~~~~~

Every new feature or bug fix should include tests:

.. code-block:: python

   # tests/test_new_feature.py

   import pytest
   import lychrel

   def test_new_function_basic():
       """Test basic functionality."""
       result = lychrel.new_function(42)
       assert result == expected_value

   def test_new_function_edge_cases():
       """Test edge cases."""
       assert lychrel.new_function(0) == 0
       assert lychrel.new_function(1) == 1

   def test_new_function_errors():
       """Test error conditions."""
       with pytest.raises(ValueError):
           lychrel.new_function(-1)

   @pytest.mark.parametrize("input,expected", [
       (1, 1),
       (10, 10),
       (100, 100),
   ])
   def test_new_function_parametrized(input, expected):
       """Test multiple cases."""
       assert lychrel.new_function(input) == expected

4. Format Code
~~~~~~~~~~~~~~

**Python:**

.. code-block:: console

   black .
   isort .

**Rust:**

.. code-block:: console

   cargo fmt
   cargo clippy

5. Run Tests
~~~~~~~~~~~~

.. code-block:: console

   # Run all tests
   pytest

   # Run specific test file
   pytest tests/test_new_feature.py

   # Run with coverage
   pytest --cov=lychrel

   # Run benchmarks
   pytest -m benchmark

6. Commit Changes
~~~~~~~~~~~~~~~~~

Write clear, descriptive commit messages:

.. code-block:: console

   git add .
   git commit -m "Add new algorithm for computing XYZ

   - Implement algorithm in Rust for performance
   - Add comprehensive tests
   - Update documentation with examples
   - Benchmark shows 30x speedup vs Python"

7. Push and Create PR
~~~~~~~~~~~~~~~~~~~~~

.. code-block:: console

   git push origin feature/add-new-algorithm

Then open a Pull Request on GitHub with:

* Clear description of changes
* Link to related issue (if any)
* Screenshots or examples (if applicable)
* Checklist of completed items

Code Style Guidelines
---------------------

Python Code Style
~~~~~~~~~~~~~~~~~

* Follow **PEP 8**
* Use **type hints** where possible
* Line length: **79 characters** (enforced by black)
* Use **docstrings** for all public functions
* Prefer **explicit over implicit**

.. code-block:: python

   from typing import Optional, Tuple

   def example_function(number: int, max_iter: Optional[int] = None) -> Tuple[int, int]:
       """Short one-line summary.

       Longer description if needed. Explain what the function does,
       any important algorithms or edge cases.

       Args:
           number: The input number to process
           max_iter: Maximum iterations (default: None means use default)

       Returns:
           A tuple of (result, iterations)

       Raises:
           ValueError: If number is negative

       Example:
           >>> example_function(42)
           (84, 1)
       """
       if number < 0:
           raise ValueError("number must be non-negative")

       iterations = 0
       result = number * 2

       return result, iterations

Rust Code Style
~~~~~~~~~~~~~~~~

* Follow **Rust conventions**
* Use **cargo fmt** for formatting
* Run **cargo clippy** for lints
* Add **comprehensive documentation**
* Use **descriptive variable names**

.. code-block:: rust

   use pyo3::prelude::*;
   use num_bigint::BigUint;

   /// Compute something interesting with a number.
   ///
   /// This function implements the XYZ algorithm which...
   ///
   /// # Arguments
   ///
   /// * `number` - The input number
   /// * `max_iterations` - Maximum iterations (default: 10000)
   ///
   /// # Returns
   ///
   /// Returns a tuple of (result, iteration_count)
   ///
   /// # Errors
   ///
   /// Returns `PyValueError` if max iterations is reached
   ///
   /// # Example
   ///
   /// ```python
   /// import lychrel
   /// result, iters = lychrel.example_function(42)
   /// ```
   #[pyfunction]
   #[pyo3(signature = (number, max_iterations=None))]
   fn example_function(
       number: BigUint,
       max_iterations: Option<usize>,
   ) -> PyResult<(BigUint, usize)> {
       let max_iter = max_iterations.unwrap_or(10000);

       for iteration in 0..max_iter {
           // Implementation
           if some_condition {
               return Ok((result, iteration));
           }
       }

       Err(PyValueError::new_err("Maximum iterations reached"))
   }

Documentation Style
~~~~~~~~~~~~~~~~~~~

* Use **reStructuredText** for Sphinx docs
* Include **examples** for all features
* Add **mathematical notation** where helpful (using math directive)
* Provide **clear explanations** of algorithms
* Link to **external resources** (Wikipedia, papers, etc.)

Testing Guidelines
------------------

Test Coverage
~~~~~~~~~~~~~

* Aim for **>90% code coverage**
* Test **all public functions**
* Include **edge cases**
* Test **error conditions**
* Add **regression tests** for bug fixes

Test Organization
~~~~~~~~~~~~~~~~~

.. code-block:: text

   tests/
   ├── test_lychrel_numbers.py    # Lychrel-specific tests
   ├── test_fibonacci.py           # Fibonacci tests
   ├── test_kaprekar.py            # Kaprekar tests
   ├── test_collatz.py             # Collatz tests
   └── test_look_and_say.py        # Look-and-say tests

Writing Good Tests
~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import pytest
   import lychrel

   class TestNewFeature:
       """Test suite for new feature."""

       def test_basic_functionality(self):
           """Test that basic usage works."""
           result = lychrel.new_function(42)
           assert result == expected_value

       def test_edge_case_zero(self):
           """Test behavior with zero."""
           result = lychrel.new_function(0)
           assert result == 0

       def test_edge_case_large_number(self):
           """Test with very large numbers."""
           large_num = 10**100
           result = lychrel.new_function(large_num)
           assert isinstance(result, int)

       def test_invalid_input(self):
           """Test that invalid input raises appropriate error."""
           with pytest.raises(ValueError, match="must be positive"):
               lychrel.new_function(-1)

       @pytest.mark.parametrize("input,expected", [
           (1, 1),
           (10, 10),
           (100, 100),
           (1000, 1000),
       ])
       def test_multiple_cases(self, input, expected):
           """Test multiple input/output pairs."""
           assert lychrel.new_function(input) == expected

Benchmark Tests
~~~~~~~~~~~~~~~

Add benchmarks to verify performance:

.. code-block:: python

   import pytest
   import lychrel
   import lychrel.py

   @pytest.mark.benchmark
   def test_performance_comparison():
       """Verify Rust implementation is faster than Python."""
       import time

       # Test input
       n = 89
       iterations = 1000

       # Rust version
       start = time.perf_counter()
       for _ in range(iterations):
           lychrel.new_function(n)
       rust_time = time.perf_counter() - start

       # Python version
       start = time.perf_counter()
       for _ in range(iterations):
           lychrel.py.new_function(n)
       python_time = time.perf_counter() - start

       # Rust should be faster
       assert python_time > rust_time

Pull Request Checklist
----------------------

Before submitting a PR, ensure:

☑ Code follows style guidelines (black, isort, cargo fmt, clippy)

☑ All tests pass locally

☑ New tests added for new functionality

☑ Documentation updated (docstrings, RST files)

☑ CHANGELOG updated (if applicable)

☑ No unnecessary changes (keep PR focused)

☑ Commit messages are clear and descriptive

☑ PR description explains what and why

☑ Linked to related issue (if applicable)

Review Process
--------------

1. **Automated Checks**: CI will run tests, linting, and coverage
2. **Code Review**: Maintainers will review your code
3. **Feedback**: Address any requested changes
4. **Approval**: Once approved, your PR will be merged
5. **Release**: Changes will be included in the next release

Be patient and responsive to feedback. All contributions are appreciated!

Adding New Algorithms
---------------------

If you want to add a new algorithm:

1. **Open an Issue First**

   Discuss the algorithm, its use cases, and API design

2. **Implement in Rust**

   Add function to ``src/lib.rs`` with PyO3 bindings

3. **Add Python Reference**

   Add pure Python version to ``lychrel/py.py`` for testing

4. **Write Comprehensive Tests**

   Include unit tests, integration tests, and benchmarks

5. **Document Thoroughly**

   * Add docstrings to functions
   * Create or update RST documentation
   * Add examples to usage guide
   * Update README.md

6. **Update Module Export**

   Add function to module's ``__all__`` list

Example PR Structure
~~~~~~~~~~~~~~~~~~~~

.. code-block:: text

   Add Euler's Totient Function

   Implements Euler's totient function φ(n) which counts the positive
   integers up to n that are relatively prime to n.

   Changes:
   - Add totient() function in Rust (src/lib.rs)
   - Add Python reference implementation (lychrel/py.py)
   - Add comprehensive tests (tests/test_totient.py)
   - Add documentation (docs/source/algorithms.rst)
   - Update README with example
   - Benchmark shows 35x speedup vs Python

   Closes #123

Community Guidelines
--------------------

* **Be respectful** and professional
* **Be patient** with reviewers and other contributors
* **Be constructive** in code reviews
* **Be helpful** to other contributors
* **Be open** to feedback and suggestions

Code of Conduct
~~~~~~~~~~~~~~~

We follow the `Python Community Code of Conduct <https://www.python.org/psf/conduct/>`_.
In summary:

* Be welcoming and inclusive
* Be respectful of differing viewpoints
* Accept constructive criticism gracefully
* Focus on what is best for the community
* Show empathy towards others

Questions?
----------

* **GitHub Issues**: For bug reports and feature requests
* **GitHub Discussions**: For questions and general discussion
* **Email**: sanfra90@gmail.com for private inquiries

Thank You!
----------

Thank you for contributing to Lychrel! Every contribution, no matter how small,
helps make the project better for everyone.

Happy coding! 🚀
