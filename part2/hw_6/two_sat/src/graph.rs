use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct DirectedEdge {
	pub a: usize,
	pub b: usize,
}

impl DirectedEdge {
	pub fn new(a: usize, b: usize) -> DirectedEdge {
		DirectedEdge { a: a, b: b }
	}
}

pub struct NodeBuilder {
	pub index: usize,
	pub edge_count: usize,
}

impl NodeBuilder {
	pub fn new(index: usize) -> NodeBuilder {
		NodeBuilder { index: index, edge_count: 0 }
	}

	pub fn to_node(&self) -> Node {
		Node::new(self)
	}
}

#[derive(Debug)]
pub struct Node {
	pub index: usize,
	pub edges: Vec<DirectedEdge>,
}

impl Node {
	pub fn new(builder: &NodeBuilder) -> Node {
		Node {
			index: builder.index,
			edges: Vec::<DirectedEdge>::with_capacity(builder.edge_count),
		}
	}
}
