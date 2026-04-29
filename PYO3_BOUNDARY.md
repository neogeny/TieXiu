Grammar Engine Architecture: Context, Caching, and PyO3

This report outlines the refined architectural split between the core library logic and its consumers (CLI/Python), focusing on the "Handle" pattern to manage memory without leaking implementation details like Arc or Sync.

1. Core Library: The Context Pattern

Instead of a global singleton, the library uses a GrammarContext. This struct acts as the owner of the grammar cache and the primary entry point for the "entourage" of parsing tools.

Key Benefits

• No Global Locks: Eliminates the need for a static Mutex, preventing bottlenecks.

• Resource Lifecycle: The cache is dropped automatically when the context goes out of scope.

• Internal Simplicity: By moving away from global statics, internal structures can use Box<T> instead of Arc<T>, provided they aren't shared across threads manually.

2. Shared Ownership with Arc::make_mut

When the library does require shared ownership (e.g., within a multi-threaded parser), Arc is used with a Copy-on-Write (CoW) strategy.

The make_mut Logic

Arc::make_mut(&mut x) provides mutable access to the inner data:

• Unique Owner: Returns a mutable reference directly (zero-cost).

• Shared Owner: Silently clones the data before returning the reference, ensuring the "original" (e.g., in the cache) remains immutable.

3. Crossing the PyO3 Boundary

To expose the Rust GrammarContext to Python, we use a wrapper pattern that leverages Python’s garbage collector as the primary owner.

The Wrapper Pattern

[rust]
#[pyclass]
pub struct PyGrammarContext {
    pub inner: GrammarContext,
}

#[pymethods]
impl PyGrammarContext {
    #[new]
    fn new() -> Self {
        PyGrammarContext { inner: GrammarContext::new() }
    }

    fn compile(&mut self, text: &str) -> PyResult<String> {
        let grammar = self.inner.get_or_compile(text);
        Ok(grammar.name.clone())
    }
}


Passing Handles Back to Rust

When Python passes a PyGrammarContext back to a Rust function, PyO3 performs a zero-copy "borrow" using the Global Interpreter Lock (GIL) for synchronization.

[rust]
#[pyfunction]
fn run_parser(context_handle: PyRefMut<'_, PyGrammarContext>, input: &str) {
    // Rust borrows its own struct back from the Python-managed memory
    let context = &mut context_handle.inner;
    context.parse(input);
}


4. Implementation Checklist

1. Purge Rc and RefCell: Ensure all internal types are Send + Sync (using Arc and Mutex where necessary) to allow them to live inside the Python-managed heap.

2. Handle-Based API: Ensure the ui module and python module both instantiate a Context rather than reaching for a global variable.

3. Feature Flags: Use Cargo.toml to make pyo3 and clap optional dependencies to keep the core library lean.

