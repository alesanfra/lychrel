use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

fn _reverse_and_add(number: u128) -> u128 {
    let reversed: u128 = number.to_string().chars().rev().collect::<String>().parse().unwrap();
    number + reversed
}

fn _is_palindrome_base_10(number: u128) -> bool {
    let string_number = number.to_string();
    string_number == string_number.chars().rev().collect::<String>()
}

fn _find_palindrome(number: u128) -> (u128, usize) {
    let mut next: u128 = number;
    let mut iterations: usize = 0;
    loop {
        if _is_palindrome_base_10(next) {
            break;
        }
        next = _reverse_and_add(next);
        iterations += 1;
    }
    (next, iterations)
}

/// Perform reverse-and-add process
#[pyfunction]
fn reverse_and_add(number: u128) -> PyResult<u128> {
    Ok(_reverse_and_add(number))
}

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn find_palindrome(number: u128) -> PyResult<u128> {
    let (palindrome, _) = _find_palindrome(number);
    Ok(palindrome)
}

/// Find the first palindrome produced by reverse-and-add routine
#[pyfunction]
fn find_palindrome_with_iterations(number: u128) -> PyResult<(u128, usize)> {
    Ok(_find_palindrome(number))
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
