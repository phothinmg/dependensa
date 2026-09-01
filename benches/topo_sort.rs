use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dependensa::graph::sort::{topo_sort, topo_sort_kahn};
use indexmap::IndexMap;

fn bench_topo_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("topo_sort");

    for n in [10, 100, 1000, 5000] {
        let graph = build_chain_graph(n);
        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("dfs_chain", n), &graph, |b, g| {
            b.iter(|| topo_sort(g))
        });
        group.bench_with_input(BenchmarkId::new("kahn_chain", n), &graph, |b, g| {
            b.iter(|| topo_sort_kahn(g))
        });
    }

    for n in [10, 100, 1000, 5000] {
        let graph = build_wide_graph(n);
        group.throughput(criterion::Throughput::Elements(n as u64));
        group.bench_with_input(BenchmarkId::new("dfs_wide", n), &graph, |b, g| {
            b.iter(|| topo_sort(g))
        });
        group.bench_with_input(BenchmarkId::new("kahn_wide", n), &graph, |b, g| {
            b.iter(|| topo_sort_kahn(g))
        });
    }

    group.finish();
}

/// A linear chain: node `i` depends on node `i+1`.
fn build_chain_graph(n: usize) -> IndexMap<String, Vec<String>> {
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

/// A wide graph: one root depends on all others.
fn build_wide_graph(n: usize) -> IndexMap<String, Vec<String>> {
    let mut g = IndexMap::new();
    let root_deps: Vec<String> = (1..n).map(|i| format!("node{i}")).collect();
    g.insert("node0".to_string(), root_deps);
    for i in 1..n {
        g.insert(format!("node{i}"), vec![]);
    }
    g
}

criterion_group!(benches, bench_topo_sort);
criterion_main!(benches);
