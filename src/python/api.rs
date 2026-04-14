use pyo3::prelude::*;

#[pyfunction]
pub fn parse_py(grammar: &str) -> PyResult<Py<PyAny>> {
    let tree = crate::api::parse(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    crate::python::tree::tree_to_py(tree)
}

#[pyfunction]
pub fn parse_to_json_py(grammar: &str) -> PyResult<String> {
    let result = crate::api::parse_as_json(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn compile_py(grammar: &str) -> PyResult<Py<PyAny>> {
    let _ = grammar;
    Err(pyo3::exceptions::PyNotImplementedError::new_err(
        "compile returns Grammar, not yet bindable",
    ))
}

#[pyfunction]
pub fn compile_to_json_py(grammar: &str) -> PyResult<String> {
    let result = crate::api::compile_as_json(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn load_json_py(json: &str) -> PyResult<Py<PyAny>> {
    let _ = json;
    Err(pyo3::exceptions::PyNotImplementedError::new_err(
        "load returns Grammar, not yet bindable",
    ))
}

#[pyfunction]
pub fn pretty(grammar: &str) -> PyResult<String> {
    let result = crate::api::pretty(grammar)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn load_boot() -> PyResult<Py<PyAny>> {
    Err(pyo3::exceptions::PyNotImplementedError::new_err(
        "load_boot returns Grammar, not yet bindable",
    ))
}

#[pyfunction]
pub fn load_boot_as_json_py() -> PyResult<String> {
    let result = crate::api::load_boot_as_json()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}

#[pyfunction]
pub fn boot_grammar_as_json_py() -> PyResult<String> {
    let result = crate::api::boot_grammar_json()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(result)
}
