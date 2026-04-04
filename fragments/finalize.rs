/// Finalizes a Cst by calling into Python to create the final Node object.
pub fn finalize_to_python(
    py: Python<'_>,
    tree: Cst,
    type_name: &str,
    factory: &PyAny // The Python 'get_node' function
) -> PyResult<PyObject> {
    let value = match tree {
        Cst::Item(node) => {
            // Attempt to treat the node as something that implements ToPy
            // In a real implementation, we'd use a more sophisticated
            // downcast or a registry.
            node.downcast_ref::<String>()
                .map(|s| s.into_py(py))
                .unwrap_or_else(|| py.None())
        }
        Cst::List(vec) => {
            let list = PyList::empty(py);
            for node in vec {
                // Recursive step: process inner nodes
                list.append(node_to_py(py, node)?)?;
            }
            list.into_py(py)
        }
    };

    // Call Python: get_node(typename, value)
    factory.call1((type_name, value))
}
