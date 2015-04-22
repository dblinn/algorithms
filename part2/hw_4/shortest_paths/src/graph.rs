use std::ops::Add;

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

pub struct NodeBuilder {
	pub index: usize,
	pub outgoing_edge_count: usize,
	pub incoming_edge_count: usize,
}

impl NodeBuilder {
	pub fn new(index: usize) -> NodeBuilder {
		NodeBuilder { index: index, outgoing_edge_count: 0, incoming_edge_count: 0 }
	}

	pub fn to_node(&self) -> Node {
		Node::new(self)
	}
}

#[derive(Debug)]
pub struct Node {
	pub index: usize,
	pub out_edges: Vec<DirectedEdge>,
	pub in_edges: Vec<DirectedEdge>,
}

impl Node {
	pub fn new(builder: &NodeBuilder) -> Node {
		Node { index: builder.index,
			out_edges: Vec::<DirectedEdge>::with_capacity(builder.outgoing_edge_count),
			in_edges: Vec::<DirectedEdge>::with_capacity(builder.incoming_edge_count)
		}
	}
}

pub enum PathLength {
	Reach(i32),
	Unreach
}

impl Add for PathLength {
	type Output = PathLength;

	fn add(self, _rhs: PathLength) -> PathLength {
		match (self, _rhs) {
			(PathLength::Reach(self_value), PathLength::Reach(rhs_value)) => PathLength::Reach(self_value + rhs_value),
			(PathLength::Unreach, _) => PathLength::Unreach,
			(_, PathLength::Unreach) => PathLength::Unreach,
		}
	}
}
