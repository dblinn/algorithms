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

#[derive(Clone, Copy)]
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

	// These data are used to augment nodes so their Strongly Connected Component can be computed
	pub scc_visit: i32,
	pub scc_lowlink: i32,
	pub scc_on_stack: bool,
}

impl Node {
	pub fn new(builder: &NodeBuilder) -> Node {
		Node {
			index: builder.index,
			edges: Vec::<DirectedEdge>::with_capacity(builder.edge_count),

			scc_visit: -1,
			scc_lowlink: -1,
			scc_on_stack: false,
		}
	}

	pub fn build(index: usize, edges: Vec<DirectedEdge>) -> Node {
		Node {
			index: index,
			edges: edges,

			scc_visit: -1,
			scc_lowlink: -1,
			scc_on_stack: false,
		}
	}

	pub fn scc_data(&self) -> (i32, i32, bool) {
		(self.scc_visit, self.scc_lowlink, self.scc_on_stack)
	}

	pub fn scc_index(&self) -> i32 {
		self.scc_lowlink
	}
}
