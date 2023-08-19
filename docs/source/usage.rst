Usage
=====

.. _installation:

Installation
------------

To use Lychrel, first install it using pip:

.. code-block:: console

   (.venv) $ pip install lychrel

Check lychrel numbers
---------------------

To check whether a number is a lychrel candidate
you can use the ``lychrel.is_lychrel_candidate()`` function:

.. autofunction:: lychrel.is_lychrel_candidate

For example:

>>> import lychrel
>>> lychrel.is_lychrel_candidate(196)
True
