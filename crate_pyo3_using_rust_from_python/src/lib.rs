use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn return_map() -> PyResult<HashMap<String, u32>> {
    let mut map = HashMap::new();
    map.insert(String::from("world"), 1);
    Ok(map)
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(return_map, m)?)?;

    Ok(())
}
