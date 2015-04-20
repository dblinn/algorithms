#[derive(Debug, Clone, Copy)]
pub struct DirectedEdge {
	pub weight: i32,
	pub a: usize,
	pub b: usize,
}

impl DirectedEdge {
	pub fn new(a: usize, b: usize, weight: i32) -> DirectedEdge {
		DirectedEdge { weight: weight, a: a, b: b }
	}
}

#[derive(Debug)]
pub struct Node {
	pub index: usize,
	pub edges: Vec<DirectedEdge>,
}

impl Node {
	pub fn new(index: usize, initial_edge_count: usize) -> Node {
		Node { index: index, edges: Vec::<DirectedEdge>::with_capacity(initial_edge_count) }
	}

	pub fn finalize_edges(&mut self) {
		self.edges.shrink_to_fit();
	}
}