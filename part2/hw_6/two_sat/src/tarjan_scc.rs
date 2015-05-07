use super::graph::*;
use std::cmp;

pub struct Tarjan<'a> {
	pub nodes: &'a mut Vec<Node>,
	pub scc_visit: i32,
}

// See http://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
impl <'a>Tarjan<'a> {
	pub fn new(nodes: &mut Vec<Node>) -> Tarjan {
		Tarjan { nodes: nodes, scc_visit: -1 }
	}

	pub fn compute_scc(&mut self) {
		self.scc_visit = 0;
		let mut stack = Vec::with_capacity(self.nodes.len() / 2);

		for i in 0 .. self.nodes.len() {
			if self.nodes[i].scc_visit < 0 {
				self.strong_connect(i, &mut stack);
			}
		}
	}

	fn strong_connect(&mut self, node_index: usize, stack: &mut Vec<usize>) {
		{
			let ref mut node = &mut self.nodes[node_index];
			node.scc_visit = self.scc_visit;
			node.scc_lowlink = self.scc_visit;
			node.scc_on_stack = true;
			stack.push(node.index);
			self.scc_visit += 1;
		}

		let edges_copy = self.nodes[node_index].edges.clone();
		for edge in edges_copy.iter() {
			let (neighbor_scc_visit, _, neighbor_scc_on_stack) = self.nodes[edge.b].scc_data();

			if neighbor_scc_visit < 0 {
				self.strong_connect(edge.b, stack);
				let neighbor_low_link = self.nodes[edge.b].scc_lowlink;
				let ref mut node = &mut self.nodes[node_index];
				node.scc_lowlink = cmp::min(node.scc_lowlink, neighbor_low_link);
			}
			else if neighbor_scc_on_stack {
				let neighbor_index = self.nodes[edge.b].scc_visit;
				let ref mut node = &mut self.nodes[node_index];
				node.scc_lowlink = cmp::min(node.scc_lowlink, neighbor_index);
			}
		}

		let (node_scc_visit, node_scc_lowlink, _) = self.nodes[node_index].scc_data();
		if node_scc_visit == node_scc_lowlink {
			let mut popped_node_index = stack.pop().unwrap();
			self.nodes[popped_node_index].scc_on_stack = false;

			while popped_node_index != node_index {
				popped_node_index = stack.pop().unwrap();
				self.nodes[popped_node_index].scc_on_stack = false;
			}
		}
	}
}

#[test]
fn test_compute_scc() {
	// See example at http://www.geeksforgeeks.org/tarjan-algorithm-find-strongly-connected-components/
	let mut n0 = Node::build(0, vec![DirectedEdge { a: 0, b: 2 }, DirectedEdge { a: 0, b: 3 }]);
	let mut n1 = Node::build(1, vec![DirectedEdge { a: 1, b: 0 }]);
	let mut n2 = Node::build(2, vec![DirectedEdge { a: 2, b: 1 }]);
	let mut n3 = Node::build(3, vec![DirectedEdge { a: 3, b: 4 }]);
	let mut n4 = Node::build(4, vec![]);

	let mut nodes = vec![n0, n1, n2, n3, n4];
	{
		let mut t = Tarjan { nodes: &mut nodes, scc_visit: 0};
		t.compute_scc();
	}

	println!("{:?}", nodes.iter().map(|node| { node.scc_index() }).collect::<Vec<i32>>());
	assert_eq!(nodes[0].scc_index(), nodes[1].scc_index());
	assert_eq!(nodes[0].scc_index(), nodes[2].scc_index());
	assert!(nodes[0].scc_index() != nodes[3].scc_index());
	assert!(nodes[0].scc_index() != nodes[4].scc_index());
	assert!(nodes[3].scc_index() != nodes[4].scc_index());
}