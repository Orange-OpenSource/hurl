#[macro_use]
extern crate criterion;

use criterion::Criterion;
use libxml::parser::Parser;
use libxml::readonly::RoNode;
use libxml::tree::{Node, NodeType};
use rayon::prelude::*;

// -- workhorse functions
// not *quite* classic depth-first search, since we keep all children at the current level in memory,
// but certainly DFS-order for traversal

fn dfs_single_classic(node: Node) -> i32 {
  1 + node
    .get_child_nodes()
    .into_iter()
    .map(dfs_single_classic)
    .sum::<i32>()
}

fn dfs_single(node: RoNode) -> i32 {
  1 + node
    .get_child_nodes()
    .into_iter()
    .map(dfs_single)
    .sum::<i32>()
}

fn dfs_parallel(node: RoNode) -> i32 {
  1 + node
    .get_child_nodes()
    .into_par_iter()
    .map(dfs_parallel)
    .sum::<i32>()
}

fn dfs_single_classic_work2(node: Node) -> (i32, usize) {
  if node.get_type() == Some(NodeType::TextNode) {
    (1, node.get_content().len())
  } else {
    node
      .get_child_nodes()
      .into_iter()
      .map(dfs_single_classic_work2)
      .fold((1, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
  }
}

fn dfs_single_work2(node: RoNode) -> (i32, usize) {
  if node.get_type() == Some(NodeType::TextNode) {
    (1, node.get_content().len())
  } else {
    node
      .get_child_nodes()
      .into_iter()
      .map(dfs_single_work2)
      .fold((1, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
  }
}

fn dfs_parallel_work2(node: RoNode) -> (i32, usize) {
  if node.get_type() == Some(NodeType::TextNode) {
    (1, node.get_content().len())
  } else {
    let dfs_work = node
      .get_child_nodes()
      .into_par_iter()
      .map(dfs_parallel_work2)
      .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    (dfs_work.0 + 1, dfs_work.1)
  }
}

// --- bencher functions
// to get big.xml download, unpack and rename:
// http://www.ins.cwi.nl/projects/xmark/Assets/standard.gz
// or use your own XML sample
fn bench_single_thread_classic(c: &mut Criterion) {
  let parser = Parser::default();
  let doc = parser.parse_file("benches/big.xml").unwrap();
  c.bench_function("single thread DFS count", move |b| {
    b.iter(|| {
      let root = doc.get_root_element().unwrap();
      assert_eq!(dfs_single_classic(root), 4_690_647)
    })
  });
}

fn bench_single_thread_classic_work2(c: &mut Criterion) {
  let parser = Parser::default();
  let doc = parser.parse_file("benches/big.xml").unwrap();
  c.bench_function("single thread DFS count+length", move |b| {
    b.iter(|| {
      let root = doc.get_root_element().unwrap();
      assert_eq!(dfs_single_classic_work2(root), (4_690_647, 81_286_567))
    })
  });
}

fn bench_single_thread(c: &mut Criterion) {
  let parser = Parser::default();
  let doc = parser.parse_file("benches/big.xml").unwrap();
  c.bench_function("read-only single thread DFS count", move |b| {
    b.iter(|| {
      let root = doc.get_root_readonly().unwrap();
      assert_eq!(dfs_single(root), 4_690_647)
    })
  });
}

fn bench_single_thread_work2(c: &mut Criterion) {
  let parser = Parser::default();
  let doc = parser.parse_file("benches/big.xml").unwrap();
  c.bench_function("read-only single thread DFS count+length", move |b| {
    b.iter(|| {
      let root = doc.get_root_readonly().unwrap();
      assert_eq!(dfs_single_work2(root), (4_690_647, 81_286_567))
    })
  });
}

fn bench_multi_thread(c: &mut Criterion) {
  let parser = Parser::default();
  let doc = parser.parse_file("benches/big.xml").unwrap();
  c.bench_function("read-only multi thread DFS count", move |b| {
    b.iter(|| {
      let root = doc.get_root_readonly().unwrap();
      assert_eq!(dfs_parallel(root), 4_690_647);
    })
  });
}

fn bench_multi_thread_work2(c: &mut Criterion) {
  let parser = Parser::default();
  let doc = parser.parse_file("benches/big.xml").unwrap();
  c.bench_function("read-only multi thread DFS count+length", move |b| {
    b.iter(|| {
      let root = doc.get_root_readonly().unwrap();
      assert_eq!(dfs_parallel_work2(root), (4_690_647, 81_286_567))
    })
  });
}
criterion_group!(
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = bench_single_thread_classic,  bench_single_thread_classic_work2, bench_single_thread, bench_single_thread_work2, bench_multi_thread, bench_multi_thread_work2
);

criterion_main!(benches);
