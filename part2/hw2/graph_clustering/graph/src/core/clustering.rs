use core::graph::Graph;
use core::node::Node;
use core::union_find::UnionFind;

struct Clustering {
	graph: Graph,
	clusters: Vec<UnionFind<Node>>,
}