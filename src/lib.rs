use num_bigint::BigUint;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


fn _reverse(number: &str) -> usize {
    number.to_string().chars().rev().collect::<String>().parse().unwrap()
}

fn _is_palindrome(string: &str) -> bool {
    string.chars().eq(string.chars().rev())
}

fn _find_palindrome(number: usize) -> (usize, usize) {
    let mut next: usize = number;
    let mut iterations: usize = 0;
    loop {
        let s = next.to_string();
        if _is_palindrome(&s) {
            break;
        }
        next += _reverse(&s);
        iterations += 1;
    }
    (next, iterations)
}

/// Perform reverse-and-add process
#[pyfunction]
fn reverse_and_add(number: usize) -> PyResult<usize> {
    Ok(number + _reverse(&number.to_string()))
}

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn find_palindrome(number: usize) -> PyResult<usize> {
    let (palindrome, _) = _find_palindrome(number);
    Ok(palindrome)
}

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn find_palindrome_with_iterations(number: usize) -> PyResult<(usize, usize)> {
    Ok(_find_palindrome(number))
}

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn echo(number: BigUint) -> PyResult<BigUint> {
    Ok(number)
}

/// A collection of functions to play with Lychrel numbers
#[pymodule]
fn lychrel(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_function(wrap_pyfunction!(reverse_and_add, module)?)?;
    module.add_function(wrap_pyfunction!(find_palindrome, module)?)?;
    module.add_function(wrap_pyfunction!(find_palindrome_with_iterations, module)?)?;
    module.add_function(wrap_pyfunction!(echo, module)?)?;
    Ok(())
}
