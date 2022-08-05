use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

mod util;

const BASE: u32 = 10;
const MAX_ITERATIONS: usize = 10000;

/// Find the first palindrome produced by reverse-and-add routine (including number of iterations
/// needed)
#[pyfunction]
fn find_lychrel_palindrome(
    number: BigUint,
    max_iterations: Option<usize>,
) -> PyResult<(BigUint, usize)> {
    let max_iterations = max_iterations.unwrap_or(MAX_ITERATIONS);
    let mut next: BigUint = number;
    let mut iterations: usize = 0;

    while iterations < max_iterations {
        let base10_representation = next.to_radix_le(BASE);

        // Check whether the decimal representation is palindrome
        if base10_representation
            .iter()
            .eq(base10_representation.iter().rev())
        {
            break;
        }

        // Reverse and add
        next += BigUint::from_radix_be(&base10_representation, BASE).unwrap();
        iterations += 1;
    }

    if iterations == max_iterations {
        Err(PyValueError::new_err("Maximum iteration reached"))
    } else {
        Ok((next, iterations))
    }
}

/// Check whether the input is a possible Lychrel number
#[pyfunction]
fn is_lychrel_candidate(number: BigUint, iterations: Option<usize>) -> bool {
    find_lychrel_palindrome(number, iterations).is_err()
}

/// Generalized Fibonacci sequence (aka Lucas sequence)
/// The number F(n) is computed with the following  formula: F(n) = p*F(n-1) - q*F(n-2),
/// where p and q are two integers. With p==1 and q==-1 we have the standard Fibonacci sequence:
/// F(n) = 1*F(n-1) - (-1)*F(n-2) = F(n-1) + F(n-2)
#[pyfunction]
fn fibonacci(number: usize, p: Option<isize>, q: Option<isize>) -> BigInt {
    if number <= 1 {
        number.to_bigint().unwrap()
    } else {
        let lucas_p = p.unwrap_or(1);
        let lucas_q = q.unwrap_or(-1);

        let mut previous = BigInt::zero();
        let mut current = BigInt::one();

        for _ in 1..number {
            let next_previous = current.clone();
            let next_current = (current * lucas_p) - (previous * lucas_q);
            previous = next_previous;
            current = next_current;
        }

        current
    }
}

/// Given a number, this function compute the sequence of digits resulting from reading out loud
/// the number, grouping together multiples of the same digit if any.
/// E.g. 3211 becomes 131221 (one 3, one 2, two 1s)
#[pyfunction]
fn read_out_loud(number: BigUint) -> PyResult<BigUint> {
    let mut current_digit: u8 = 0;
    let mut count: u8 = 0;
    let mut result: Vec<u8> = Vec::new();

    for i in number.to_radix_be(BASE) {
        if count == 0 {
            current_digit = i;
        }

        if i == current_digit {
            count += 1;
        } else {
            result.push(count);
            result.push(current_digit);
            current_digit = i;
            count = 1;
        }
    }

    result.push(count);
    result.push(current_digit);

    BigUint::from_radix_be(&result, BASE).ok_or_else(|| {
        PyValueError::new_err(format!("Unable to read out loud the number {}", number))
    })
}

/// Kaprekar's routine
#[pyfunction]
fn kaprekar(
    number: BigUint,
    base: Option<u32>,
    max_iterations: Option<usize>,
) -> PyResult<BigUint> {
    let _base = base.unwrap_or(BASE);
    let _max_iterations = max_iterations.unwrap_or(MAX_ITERATIONS);
    let mut previous = number;

    for _ in 0.._max_iterations {
        let sorted = util::sorted_digits(&previous, _base);
        let first = BigUint::from_radix_le(&sorted, _base)
            .ok_or_else(|| PyValueError::new_err("Not a decimal number"))?;
        let second = BigUint::from_radix_be(&sorted, _base)
            .ok_or_else(|| PyValueError::new_err("Not a decimal number"))?;
        let result = first - second;

        if result == previous {
            return Ok(result);
        }

        previous = result;
    }

    Err(PyValueError::new_err("Maximum iteration reached"))
}

/// Collatz conjecture sequence
#[pyfunction]
fn collatz(start: u128) -> PyResult<Vec<u128>> {
    if start == 0 {
        Err(PyValueError::new_err("Start number must be > 0"))
    } else {
        let mut result: Vec<u128> = vec![start];
        let mut current: u128 = start;

        while current != 1 {
            current = if current % 2 != 0 {
                current * 3 + 1
            } else {
                current / 2
            };
            result.push(current);
        }

        Ok(result)
    }
}

/// A collection of functions to play with Lychrel numbers and other funny mathematical problems
#[pymodule]
fn lychrel(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_function(wrap_pyfunction!(is_lychrel_candidate, module)?)?;
    module.add_function(wrap_pyfunction!(find_lychrel_palindrome, module)?)?;
    module.add_function(wrap_pyfunction!(fibonacci, module)?)?;
    module.add_function(wrap_pyfunction!(read_out_loud, module)?)?;
    module.add_function(wrap_pyfunction!(kaprekar, module)?)?;
    module.add_function(wrap_pyfunction!(collatz, module)?)?;

    Ok(())
}
