Usage
=====

.. _installation:

Installation
------------

To use Lychrel, first install it using pip:

.. code-block:: console

   pip install lychrel

Check lychrel numbers
---------------------

To check whether a number is a lychrel candidate
you can use the ``lychrel.is_lychrel_candidate()`` function:

.. autofunction:: lychrel.is_lychrel_candidate

For example:

>>> import lychrel
>>> lychrel.is_lychrel_candidate(196)
True

The ``lychrel.is_lychrel_candidate()`` function applies iteratively the *reverse-and-add* algorithm until it finds
a palindrome in base-10 representation.

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
