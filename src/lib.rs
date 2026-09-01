#[doc(hidden)]
pub mod graph;
pub use graph::{GraphObject, generate_graph};
use napi_derive::napi;
use std::path::PathBuf;

/// Run full dependency-graph analysis and return the result as a JSON string.
///
/// This is the simplest call site: the JS side can `JSON.parse()` the result
/// and inspect any field of [`crate::graph::GraphObject`].
///
/// Available in both the cargo crate (as a plain Rust function) and the
/// napi Node.js binding (as a JS function).
///
/// # Errors
///
/// Returns a napi `Error` (thrown as a JS `Error`) if the project cannot be
/// read or traversal fails.
#[napi]
pub fn analyze(entry: String, root: String) -> napi::Result<String> {
    let graph_obj = generate_graph(&entry, resolve_root(&root)?)
        .map_err(|e| napi::Error::from_reason(format!("dependensa: {e}")))?;
    serde_json::to_string(&graph_obj)
        .map_err(|e| napi::Error::from_reason(format!("dependensa: serialize: {e}")))
}

/// A structured projection of [`crate::graph::GraphObject`].
///
/// Exposes the most useful derived views as typed fields. Available in both
/// the cargo crate (as a plain Rust struct) and the napi Node.js binding
/// (as a JS object).
#[napi(object)]
pub struct GraphResult {
    /// Topologically sorted files (dependencies first).
    pub sort: Vec<String>,
    /// NPM package specifiers found in the project (e.g. `"react"`).
    pub npm: Vec<String>,
    /// Node.js built-in module specifiers (e.g. `"node:fs"`).
    pub node: Vec<String>,
    /// Warnings collected during traversal.
    pub warn: Vec<String>,
    /// Leaf files — files with no local-file dependencies.
    pub leaf: Vec<String>,
    /// Mutual dependency pairs (files that depend on each other).
    pub mutual: Vec<Vec<String>>,
    /// The graph rendered as a text tree.
    pub text_graph: String,
    /// The raw dependency map: file → its local imports.
    pub deps: std::collections::HashMap<String, Vec<String>>,
}

/// Run analysis and return a structured [`GraphResult`] object.
///
/// Prefer this over [`analyze`] when you want typed access to individual
/// views without parsing JSON on the JS side.
///
/// Available in both the cargo crate (as a plain Rust function) and the
/// napi Node.js binding (as a JS function).
#[napi]
pub fn graph(entry: String, root: String) -> napi::Result<GraphResult> {
    let g = generate_graph(&entry, resolve_root(&root)?)
        .map_err(|e| napi::Error::from_reason(format!("dependensa: {e}")))?;

    // Convert the IndexMap<String, Vec<String>> into a HashMap so napi-rs
    // can hand it to JS as a plain object.
    let deps = g
        .deps()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    Ok(GraphResult {
        sort: g.sort().to_vec(),
        npm: g.npm().to_vec(),
        node: g.node().to_vec(),
        warn: g.warn().to_vec(),
        leaf: g.leaf().to_vec(),
        mutual: g.mutual().to_vec(),
        text_graph: g.text_graph().to_string(),
        deps,
    })
}

/// Resolve a possibly-relative project root to an absolute path, mirroring
/// the behavior of the pure-Rust `generate_graph` (which canonicalizes
/// relative paths against the current working directory).
fn resolve_root(root: &str) -> napi::Result<PathBuf> {
    let p = PathBuf::from(root);
    if p.is_absolute() {
        Ok(p)
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(p))
            .map_err(|e| napi::Error::from_reason(format!("dependensa: cwd: {e}")))
    }
}
