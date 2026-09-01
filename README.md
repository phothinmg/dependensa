<div align="center">
<img src="https://susee.phothin.dev/logo/susee-bg-white.webp" width="160" height="160" alt="susee" />
  <h1>dependensa</h1>
  <p>A static analysis tool that examines TypeScript &amp; JavaScript projects and produces dependency graphs.</p>
  <p>
    <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" />
    <img alt="Rust edition" src="https://img.shields.io/badge/edition-2024-orange.svg" />
    <img alt="crates.io" src="https://img.shields.io/badge/crates.io-dependensa-red.svg" />
    <img alt="npm" src="https://img.shields.io/badge/npm-@suseejs/dependensa-red.svg" />
  </p>
</div>

---

## Overview

`dependensa` statically analyzes TypeScript and JavaScript projects to produce a complete dependency graph. Starting from an entry file, it recursively resolves all local-file imports, NPM module specifiers, and Node.js built-in module specifiers to build a rich graph object with multiple derived views:

- **Topological sort** of the dependency graph (DAG)
- **NPM** and **Node.js built-in** module lists
- **Circular dependency** detection (via DFS)
- **Mutual (two-way) dependency** pairs
- **Leaf files** (files with no local-file dependencies)
- **Dependency chains** and **entry-to-leaf** paths
- **Text-tree rendering** of the full graph

It is published in two forms from a single codebase:

| Platform         | Package               | Install                     |
| ---------------- | --------------------- | --------------------------- |
| Rust (crates.io) | `dependensa`          | `cargo add dependensa`      |
| Node.js (npm)    | `@suseejs/dependensa` | `npm i @suseejs/dependensa` |

The Node.js package ships prebuilt native binaries (via [napi-rs](https://napi.rs)) for Linux, macOS, and Windows on x64 and arm64, so no toolchain is required at install time.

## Features

- Parses JS/TS source with the [oxc](https://github.com/oxc-project/oxc) parser and AST visitor
- Handles ESM imports, dynamic `import()`, `require()`, `import = require()`, and re-exports
- Resolves file extensions (`.js`, `.cjs`, `.mjs`, `.ts`, `.mts`, `.cts`, `.jsx`, `.tsx`, `.json`) and directory `index` modules
- Reads `package.json` and `node_modules` metadata to classify dependencies
- Serializes the full graph to JSON via `serde` (Rust) / `JSON.parse` (Node.js)
- Exposes a typed `GraphResult` object to Node.js consumers via napi-rs

## Installation

**Rust** — add `dependensa` to your `Cargo.toml`:

```toml
[dependencies]
dependensa = "0.1"
```

or

```bash
cargo add dependensa
```

**Node.js** — install the native package:

```bash
npm i @suseejs/dependensa
# or
pnpm add @suseejs/dependensa
yarn add @suseejs/dependensa
```

Prebuilt binaries are available for:

| OS      | Architecture | Target           |
| ------- | ------------ | ---------------- |
| Linux   | x64 / arm64  | `gnu` and `musl` |
| macOS   | x64 / arm64  | `darwin`         |
| Windows | x64 / arm64  | `msvc`           |

Requires Node.js `>= 20.17.0`.

## Quick Start

### Rust

```rust
use dependensa::{generate_graph, GraphObject};

fn main() {
    // Analyze a project starting from "index.ts" in the current directory
    let graph: GraphObject = generate_graph("index.ts", ".").unwrap();

    // Print the full dependency tree
    println!("{}", graph.text_graph());

    // List leaf files (no local imports)
    for file in graph.leaf() {
        println!("leaf: {file}");
    }

    // Report circular dependencies
    for cycle in graph.circular() {
        println!("circular: {}", cycle.chain.join(" -> "));
    }

    // Topological order (dependencies first)
    for file in graph.sort() {
        println!("{}", file);
    }
}
```

### Node.js / TypeScript

```ts
import { analyze, graph } from "@suseejs/dependensa";

// `graph()` returns a typed object with each derived view.
const result = graph("index.ts", ".");

console.log(result.textGraph);

for (const file of result.leaf) {
  console.log("leaf:", file);
}

for (const file of result.sort) {
  console.log(file);
}

// `analyze()` returns the same data as a JSON string — handy when you
// want to pipe or log the full serialized graph.
const json = analyze("index.ts", ".");
const full = JSON.parse(json);
console.log(full);
```

## API Reference

### Rust API

#### `generate_graph`

```rust
pub fn generate_graph<P: AsRef<Path>>(entry: &str, root: P) -> std::io::Result<GraphObject>
```

Recursively traverses from `entry` (relative to `root`) and returns a [`GraphObject`].

| Argument | Description                                            |
| -------- | ------------------------------------------------------ |
| `entry`  | Entry file path relative to `root` (e.g. `"index.ts"`) |
| `root`   | Project root directory (absolute or relative to CWD)   |

#### `GraphObject`

`GraphObject` implements `serde::Serialize` so the full graph can be serialized to JSON.

| Method             | Returns                          | Description                                           |
| ------------------ | -------------------------------- | ----------------------------------------------------- |
| `sort()`           | `&[String]`                      | Topologically sorted files (dependencies first)       |
| `npm()`            | `&[String]`                      | NPM package specifiers (e.g. `"react"`)               |
| `node()`           | `&[String]`                      | Node.js built-in module specifiers (e.g. `"node:fs"`) |
| `deps()`           | `&IndexMap<String, Vec<String>>` | Raw dependency map: file → its local imports          |
| `warn()`           | `&[String]`                      | Warnings collected during traversal                   |
| `mutual()`         | `&[Vec<String>]`                 | Pairs of files that depend on each other              |
| `leaf()`           | `&[String]`                      | Files with no local-file dependencies                 |
| `circular()`       | `&[CircularDependency]`          | Circular dependency chains detected                   |
| `dependents(file)` | `Vec<String>`                    | Files that depend on the given file                   |
| `chain()`          | `&IndexMap<String, Vec<String>>` | Full dependency chains for every file                 |
| `entry_to_leaf()`  | `&[Vec<String>]`                 | Paths from the entry file to each leaf                |
| `text_graph()`     | `&str`                           | The graph rendered as a text tree                     |

#### Serialization (Rust)

```rust
use dependensa::generate_graph;

let graph = generate_graph("index.ts", ".").unwrap();
let json = serde_json::to_string_pretty(&graph).unwrap();
println!("{json}");
```

### Node.js API

The Node.js binding exposes two functions. Both take the same arguments as the Rust `generate_graph` entry point.

| Function  | Signature                       | Returns                          |
| --------- | ------------------------------- | -------------------------------- |
| `analyze` | `(entry: string, root: string)` | `string` — full graph as JSON    |
| `graph`   | `(entry: string, root: string)` | `GraphResult` — typed projection |

| Argument | Description                                            |
| -------- | ------------------------------------------------------ |
| `entry`  | Entry file path relative to `root` (e.g. `"index.ts"`) |
| `root`   | Project root directory (absolute or relative to CWD)   |

#### `GraphResult`

`GraphResult` is a typed projection of the most useful derived views. For fields not exposed here (circular dependencies, dependency chains, entry-to-leaf paths, dependents), use `analyze()` and `JSON.parse` the result.

| Field       | Type                       | Description                                           |
| ----------- | -------------------------- | ----------------------------------------------------- |
| `sort`      | `string[]`                 | Topologically sorted files (dependencies first)       |
| `npm`       | `string[]`                 | NPM package specifiers (e.g. `"react"`)               |
| `node`      | `string[]`                 | Node.js built-in module specifiers (e.g. `"node:fs"`) |
| `deps`      | `Record<string, string[]>` | Raw dependency map: file → its local imports          |
| `warn`      | `string[]`                 | Warnings collected during traversal                   |
| `mutual`    | `string[][]`               | Pairs of files that depend on each other              |
| `leaf`      | `string[]`                 | Files with no local-file dependencies                 |
| `textGraph` | `string`                   | The graph rendered as a text tree                     |

#### Serialization (Node.js)

```ts
import { analyze } from "@suseejs/dependensa";

const json = analyze("index.ts", ".");
const graph = JSON.parse(json);

// `graph.circular`, `graph.chain`, `graph.entry_to_leaf`, etc. are only
// available through the JSON form.
for (const cycle of graph.circular) {
  console.log("circular:", cycle.chain.join(" -> "));
}
```

## Module Structure

```
src/
├── lib.rs                  # Public exports + napi bindings (analyze, graph)
└── graph/
    ├── mod.rs              # GraphObject struct, generate_graph(), public API
    ├── collect.rs          # Recursive dependency collection (file traversal)
    ├── resolve_ext.rs     # Resolve file paths / directory index modules
    ├── package_info.rs    # Parse package.json & node_modules metadata
    ├── sort.rs            # Topological sort (DFS + Kahn's algorithm)
    ├── analyze.rs         # Circular dependency detection & chains (DFS)
    ├── mutual.rs          # Two-way mutual dependency detection
    ├── leaf.rs            # Leaf file detection (no local imports)
    ├── visualize.rs       # Text-tree rendering of the dependency graph
    ├── utils.rs           # Graph construction, Node.js built-in module list
    └── handlers/
        ├── mod.rs         # Re-exports collect_module_specifiers
        └── visit.rs       # oxc AST visitor for import/require specifiers

benches/                    # criterion benchmarks
├── generate_graph.rs
├── parse_specifiers.rs
├── topo_sort.rs
└── analyze.rs
```

## Development

### Rust

```bash
make check    # Cross-platform cargo check (8 targets)
make test     # Run the test suite
make fmt      # Format code
make doc      # Build and open docs
make dr       # Dry-run cargo publish
make publish  # Publish to crates.io
make bench    # Run criterion benchmarks
```

### Node.js

```bash
npm run build:debug   # napi build --platform (debug)
npm run build         # napi build --release --platform
npm run artifacts     # Collect and verify prebuilt artifacts
npm publish           # Publish @suseejs/dependensa to npm
```

### Benchmarks

Benchmarks are powered by [criterion](https://benthlelmsen.com/criterion) and live in `benches/`:

| Bench              | What it measures                               |
| ------------------ | ---------------------------------------------- |
| `generate_graph`   | End-to-end graph generation from an entry file |
| `parse_specifiers` | oxc parsing & import specifier extraction      |
| `topo_sort`        | Topological sort over the dependency map       |
| `analyze`          | Circular/mutual/leaf analysis passes           |

Run them with:

```bash
make bench
# or directly
cargo bench --bench generate_graph
```

## License

Apache-2.0
