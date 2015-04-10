use core::graph::Graph;
use core::node::Node;
use core::union_find::UnionFind;

pub struct Clustering<'a> {
	pub graph: Graph,
	pub clusters: Vec<UnionFind<'a, i32>>,
}