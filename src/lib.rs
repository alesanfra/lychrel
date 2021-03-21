use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Perform reverse-and-add process
#[pyfunction]
fn reverse_and_add(number: usize) -> PyResult<usize> {
    let reversed: usize = number.to_string().chars().rev().collect::<String>().parse().unwrap();
    Ok(number + reversed)
}


/// A collection of functions to play with Lychrel numbers
#[pymodule]
fn lychrel(py: Python, module: &PyModule) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_function(wrap_pyfunction!(reverse_and_add, module)?)?;
    Ok(())
}
