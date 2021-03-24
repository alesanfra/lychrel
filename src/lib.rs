use num_bigint::BigUint;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

const BASE: u32 = 10;
const MAX_ITERATIONS: usize = 10000;

create_exception!(lychrel, LychrelError, PyException);


/// Perform reverse-and-add process
#[pyfunction]
fn reverse_and_add(number: BigUint) -> PyResult<BigUint> {
    match BigUint::from_radix_be(&number.to_radix_le(BASE), BASE) {
        Some(reversed) => Ok(number + reversed),
        None => Err(LychrelError::new_err("Unable to reverse number"))
    }
}

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn find_palindrome(number: BigUint) -> PyResult<BigUint> {
    let (palindrome, _) = find_palindrome_with_iterations(number)?;
    Ok(palindrome)
}

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn find_palindrome_with_iterations(number: BigUint) -> PyResult<(BigUint, usize)> {
    let mut next: BigUint = number;
    let mut iterations: usize = 0;

    while iterations < MAX_ITERATIONS {
        let base10_representation = next.to_radix_le(BASE);

        // Check whether the decimal representation is palindrome
        if base10_representation.iter().eq(base10_representation.iter().rev()) {
            break;
        }

        // Reverse and add
        next += BigUint::from_radix_be(&base10_representation, BASE).unwrap();
        iterations += 1;
    }

    if iterations == MAX_ITERATIONS {
        Err(LychrelError::new_err("Maximum iteration reached"))
    } else {
        Ok((next, iterations))
    }
}

/// A collection of functions to play with Lychrel numbers
#[pymodule]
fn lychrel(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_function(wrap_pyfunction!(reverse_and_add, module)?)?;
    module.add_function(wrap_pyfunction!(find_palindrome, module)?)?;
    module.add_function(wrap_pyfunction!(find_palindrome_with_iterations, module)?)?;
    Ok(())
}
