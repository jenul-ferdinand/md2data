use pyo3::prelude::*;
use ::md2data::{convert_str, OutputFormat, ParsingMode};

#[pyfunction]
fn convert(input: &str, format: &str) -> PyResult<String> {
    let fmt = OutputFormat::from_str(format)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("invalid format (use json|yaml|toml|xml)"))?;

    convert_str(input, fmt, ParsingMode::default())
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pymodule]
fn md2data(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    Ok(())
}
