use super::graph::*;
use super::graph::PathLength::*;
use super::graph_builder::*;
use std::collections::BinaryHeap;
use std::cmp::*;

pub struct Dijkstra<'a> {
	pub shortest_paths: Vec<PathLength>,
	pub in_cut: Vec<bool>,
	pub nodes: &'a Vec<Node>,

	edge_heap: BinaryHeap<DijkstraEdge>,
	nodes_added: usize,
}

impl <'a>Dijkstra<'a> {
	pub fn new(nodes: &'a Vec<Node>, edge_count: usize) -> Dijkstra<'a> {
		let paths = Vec::<PathLength>::with_capacity(nodes.len());
		let in_cut = Vec::<bool>::with_capacity(nodes.len());
		Dijkstra { shortest_paths: paths,
			in_cut: in_cut,
			nodes: nodes,
			edge_heap: BinaryHeap::<DijkstraEdge>::with_capacity(max(edge_count - nodes.len(), 1)),
			nodes_added: 0,
		}
	}

	pub fn compute_shortest_paths(&mut self, source_index: usize) {
		self.initialize_paths(source_index);

		while self.nodes_added < self.nodes.len() && !self.edge_heap.is_empty() {
			let edge = self.edge_heap.pop().unwrap();
			if !self.in_cut[edge.b] {
				self.add_node_to_cut(&self.nodes[edge.b], edge.path_length);
			}
		}
	}

	pub fn longest_path(&self) -> PathLength {
		*self.shortest_paths.iter().max().unwrap()
	}

	fn node_count(&self) -> usize {
		self.nodes.len()
	}

	fn add_node_to_cut(&mut self, node: &Node, path_length: i32) {
		self.in_cut[node.index] = true;
		self.shortest_paths[node.index] = Reach(path_length);
		self.nodes_added += 1;

		// Add outgoing edges to edges not already in the cut to the heap
		for edge in node.out_edges.iter() {
			if !self.in_cut[edge.b] {
				println!("Adding edge {} -> {} with path_length {}", edge.a, edge.b, path_length + edge.weight);
				self.edge_heap.push(DijkstraEdge::new(path_length, edge));
			}
		}
	}

	fn initialize_paths(&mut self, source_index: usize) {
		self.shortest_paths.clear();
		self.in_cut.clear();
		self.nodes_added = 0;

		for i in 0 .. self.node_count() {
			if i == source_index {
				self.in_cut.push(true);
				self.shortest_paths.push(Reach(0));
			}
			else {
				self.in_cut.push(false);
				self.shortest_paths.push(Unreach);
			}
		}

		self.add_node_to_cut(&self.nodes[source_index], 0);
	}
}

#[test]
fn test_positive_weights() {
	let (node_count, edge_count, nodes) = build_graph_from_file("test_cases/positive_weights.txt");
	let mut dijkstra = Dijkstra::new(&nodes, edge_count);
	dijkstra.compute_shortest_paths(0);
	assert_eq!(dijkstra.shortest_paths[0], Reach(0));
	assert_eq!(dijkstra.shortest_paths[1], Reach(3));
	assert_eq!(dijkstra.shortest_paths[2], Reach(2));
	assert_eq!(dijkstra.shortest_paths[3], Reach(3));
}
