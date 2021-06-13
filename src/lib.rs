use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::exceptions::PyValueError;

const BASE: u32 = 10;
const MAX_ITERATIONS: usize = 10000;


/// Reverse the base 10 representation of the input number and return the sum.
/// E.g. reverse_and_add(23) -> 55, because 23 + 32 == 55
#[pyfunction]
fn reverse_and_add(number: BigUint) -> PyResult<BigUint> {
    match BigUint::from_radix_be(&number.to_radix_le(BASE), BASE) {
        Some(reversed) => Ok(number + reversed),
        None => Err(PyValueError::new_err("Unable to reverse number")),
    }
}

/// Find the first palindrome produced by reverse-and-add routine (including number of iterations
/// needed)
#[pyfunction]
fn lychrel_palindrome_with_iterations(
    number: BigUint,
    max_iterations: usize,
) -> PyResult<(BigUint, usize)> {
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

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn lychrel_palindrome(number: BigUint) -> PyResult<BigUint> {
    let (palindrome, _) = lychrel_palindrome_with_iterations(number, MAX_ITERATIONS)?;
    Ok(palindrome)
}

/// Returns the number of iterations needed to find the first palindrome produced by
/// reverse-and-add routine
#[pyfunction]
fn lychrel_iterations(number: BigUint) -> PyResult<usize> {
    let (_, iterations) = lychrel_palindrome_with_iterations(number, MAX_ITERATIONS)?;
    Ok(iterations)
}

/// Check whether the input is a possible Lychrel number
#[pyfunction]
fn is_lychrel_candidate(number: BigUint, iterations: Option<usize>) -> bool {
    lychrel_palindrome_with_iterations(number, iterations.unwrap_or(MAX_ITERATIONS)).is_err()
}

/// Generalized fibonacci sequence
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

/// A collection of functions to play with Lychrel numbers and other funny mathematical problems
#[pymodule]
fn lychrel(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_function(wrap_pyfunction!(reverse_and_add, module)?)?;
    module.add_function(wrap_pyfunction!(is_lychrel_candidate, module)?)?;
    module.add_function(wrap_pyfunction!(lychrel_palindrome, module)?)?;
    module.add_function(wrap_pyfunction!(lychrel_iterations, module)?)?;
    module.add_function(wrap_pyfunction!(
        lychrel_palindrome_with_iterations,
        module
    )?)?;
    module.add_function(wrap_pyfunction!(fibonacci, module)?)?;

    Ok(())
}
