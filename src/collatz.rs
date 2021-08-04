use pyo3::class::iter::IterNextOutput;
use pyo3::prelude::*;
use pyo3::PyIterProtocol;

#[pyclass]
pub struct CollatzIterator {
    next: u128,
    stop: bool,
}

#[pymethods]
impl CollatzIterator {
    #[new]
    pub fn new(start: u128) -> Self {
        Self {
            next: start,
            stop: false,
        }
    }
}

#[pyproto]
impl PyIterProtocol for CollatzIterator {
    fn __iter__(self_: PyRef<Self>) -> PyRef<Self> {
        self_
    }

    fn __next__(mut self_: PyRefMut<Self>) -> IterNextOutput<u128, ()> {
        if self_.stop {
            IterNextOutput::Return(())
        } else {
            let current = self_.next;
            self_.next = if current % 2 != 0 {
                current * 3 + 1
            } else {
                current / 2
            };
            self_.stop = current == 1;
            IterNextOutput::Yield(current)
        }
    }
}
