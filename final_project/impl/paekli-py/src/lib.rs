use paekli_core::store::DistributionStrategy;
use pyo3::{exceptions::PyValueError, prelude::*};

fn strategy_from_str(strategy: Option<String>) -> PyResult<DistributionStrategy> {
    match strategy.as_deref() {
        None => Ok(DistributionStrategy::Fs),
        Some("http") => Ok(DistributionStrategy::Http),
        Some("sql") => Ok(DistributionStrategy::Sql),
        Some("fs") => Ok(DistributionStrategy::Fs),
        Some(s) => Err(PyValueError::new_err(format!(
            "unknown distribution strategy: '{}'",
            s
        ))),
    }
}

/// Send a paekli
#[pyfunction]
fn send(receiver: String, content: String, strategy: Option<String>) -> PyResult<()> {
    let strategy = strategy_from_str(strategy)?;
    let dist_center = paekli_core::store::new_distribution_center(strategy);
    dist_center.store(receiver, content, false);
    Ok(())
}

/// Receive a paekli
#[pyfunction]
fn receive(receiver: String, strategy: Option<String>) -> PyResult<Option<String>> {
    let strategy = strategy_from_str(strategy)?;
    let dist_center = paekli_core::store::new_distribution_center(strategy);
    Ok(dist_center.retrieve(receiver))
}

/// A Python module implemented in Rust.
#[pymodule]
fn paekli_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(send, m)?)?;
    m.add_function(wrap_pyfunction!(receive, m)?)?;
    Ok(())
}
