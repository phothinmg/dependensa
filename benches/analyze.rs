use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use dependensa::graph::analyze::analyze_dependencies;
use dependensa::graph::leaf::find_leaf_files;
use dependensa::graph::mutual::find_mutual_dependencies;
use dependensa::graph::visualize::visualize_dependencies;
use indexmap::IndexMap;

fn bench_analyze(c: &mut Criterion) {
    let mut group = c.benchmark_group("analyze_dependencies");

    for n in [10, 100, 1000, 5000] {
        let graph = build_dag(n);
        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("dag", n),
            &graph,
            |b, g| b.iter(|| analyze_dependencies(g)),
        );
    }

    for n in [10, 100, 1000] {
        let graph = build_cyclic(n);
        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("cyclic", n),
            &graph,
            |b, g| b.iter(|| analyze_dependencies(g)),
        );
    }

    group.finish();

    // Derived views
    let mut group = c.benchmark_group("derived_views");

    for n in [100, 1000, 5000] {
        let graph = build_dag(n);
        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("leaf", n),
            &graph,
            |b, g| b.iter(|| find_leaf_files(g)),
        );
        group.bench_with_input(
            BenchmarkId::new("mutual", n),
            &graph,
            |b, g| b.iter(|| find_mutual_dependencies(g)),
        );
        group.bench_with_input(
            BenchmarkId::new("visualize", n),
            &graph,
            |b, g| b.iter(|| visualize_dependencies(g)),
        );
    }

    group.finish();
}

/// DAG where node `i` depends on `i+1` (a chain).
fn build_dag(n: usize) -> IndexMap<String, Vec<String>> {
    let mut g = IndexMap::new();
    for i in 0..n {
        let deps = if i + 1 < n {
            vec![format!("node{}", i + 1)]
        } else {
            vec![]
        };
        g.insert(format!("node{i}"), deps);
    }
    g
}

/// Cyclic graph: node `i` depends on `i+1`, and the last depends back on node 0.
fn build_cyclic(n: usize) -> IndexMap<String, Vec<String>> {
    let mut g = IndexMap::new();
    for i in 0..n {
        let deps = if i + 1 < n {
            vec![format!("node{}", i + 1)]
        } else {
            vec!["node0".to_string()]
        };
        g.insert(format!("node{i}"), deps);
    }
    g
}

criterion_group!(benches, bench_analyze);
criterion_main!(benches);