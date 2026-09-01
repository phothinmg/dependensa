use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dependensa::graph::handlers::collect_module_specifiers;
use std::path::Path;

fn bench_parse_specifiers(c: &mut Criterion) {
    let mut group = c.benchmark_group("collect_module_specifiers");

    for n in [10, 50, 200, 1000] {
        let source = build_source(n);
        let path = Path::new("bench.ts");

        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{n} imports")),
            &source,
            |b, src| {
                b.iter(|| collect_module_specifiers(src, path));
            },
        );
    }

    group.finish();

    // Benchmark different statement flavors.
    let mut group = c.benchmark_group("collect_module_specifiers (flavors)");
    let path = Path::new("bench.ts");

    let esm = "import a from \"./a\";\nimport { b } from \"./b\";\nimport * as c from \"./c\";\n";
    let cjs = "const a = require(\"./a\");\nconst b = require(\"./b\").b;\n";
    let dynamic = "const a = await import(\"./a\");\nconst b = await import(\"./b\");\n";
    let reexport =
        "export * from \"./a\";\nexport { x } from \"./b\";\nexport * as ns from \"./c\";\n";
    let ts_eq = "import a = require(\"./a\");\nimport b = require(\"./b\");\n";

    group.bench_function("esm", |b| b.iter(|| collect_module_specifiers(esm, path)));
    group.bench_function("cjs", |b| b.iter(|| collect_module_specifiers(cjs, path)));
    group.bench_function("dynamic", |b| {
        b.iter(|| collect_module_specifiers(dynamic, path))
    });
    group.bench_function("reexport", |b| {
        b.iter(|| collect_module_specifiers(reexport, path))
    });
    group.bench_function("ts_import_equals", |b| {
        b.iter(|| collect_module_specifiers(ts_eq, path))
    });

    group.finish();
}

fn build_source(n: usize) -> String {
    let mut s = String::new();
    for i in 0..n {
        s.push_str(&format!("import dep{i} from \"./dep{i}\";\n"));
    }
    s
}

criterion_group!(benches, bench_parse_specifiers);
criterion_main!(benches);
