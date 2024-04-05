use pyo3::prelude::*;

/// Send a paekli
#[pyfunction]
fn send(receiver: String, content: String) {
    let dist_center = paekli_core::store::new_distribution_center();
    dist_center.store(receiver, content, false);
}

/// Receive a paekli
#[pyfunction]
fn receive(receiver: String) -> Option<String> {
    let dist_center = paekli_core::store::new_distribution_center();
    dist_center.retrieve(receiver)
}

/// A Python module implemented in Rust.
#[pymodule]
fn paekli_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send, m)?)?;
    m.add_function(wrap_pyfunction!(receive, m)?)?;
    Ok(())
}
