use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use dependensa::generate_graph;
use std::fs;
use tempfile::TempDir;

fn bench_generate_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_graph");

    for n in [5, 25, 100, 500] {
        let dir = TempDir::new().unwrap();
        setup_project(dir.path(), n);

        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("project", n),
            &dir,
            |b, dir| {
                b.iter(|| {
                    generate_graph("index.ts", dir.path()).unwrap();
                });
            },
        );
    }

    group.finish();
}

/// Create a synthetic project under `root` with `n` files in a chain.
///
/// `index.ts` → `mod0.ts` → `mod1.ts` → … → `mod{n-1}.ts`
/// Each file also imports `react` (npm) and `node:fs` (node builtin).
fn setup_project(root: &std::path::Path, n: usize) {
    fs::write(
        root.join("package.json"),
        r#"{"type":"module","dependencies":{"react":"^18.0.0"}}"#,
    )
    .unwrap();

    let mut index = String::from("import * as fs from \"node:fs\";\nimport react from \"react\";\n");
    if n > 0 {
        index.push_str("import { mod0 } from \"./mod0\";\n");
    }
    fs::write(root.join("index.ts"), index).unwrap();

    for i in 0..n {
        let mut content = String::from("import * as fs from \"node:fs\";\nimport react from \"react\";\n");
        if i + 1 < n {
            content.push_str(&format!("import {{ mod{} }} from \"./mod{}\";\n", i + 1, i + 1));
        }
        content.push_str(&format!("export const mod{i} = {i};\n"));
        fs::write(root.join(format!("mod{i}.ts")), content).unwrap();
    }
}

criterion_group!(benches, bench_generate_graph);
criterion_main!(benches);