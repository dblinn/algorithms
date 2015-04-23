use super::graph::*;
use super::graph::PathLength::*;
use super::graph_builder::*;

pub struct Dijkstra<'a> {
	pub shortest_paths: Vec<PathLength>,
	pub in_cut: Vec<bool>,
	pub nodes: &'a Vec<Node>,

	nodes_added: usize,
}

impl <'a>Dijkstra<'a> {
	pub fn new(nodes: &'a Vec<Node>) -> Dijkstra<'a> {
		let paths = Vec::<PathLength>::with_capacity(nodes.len());
		let in_cut = Vec::<bool>::with_capacity(nodes.len());
		Dijkstra { shortest_paths: paths, in_cut: in_cut, nodes: nodes, nodes_added: 0 }
	}

	pub fn compute_shortest_paths(&mut self, source_index: usize) {
		self.initialize_paths(source_index);
	}

	fn node_count(&self) -> usize {
		self.nodes.len()
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
	}
}

#[test]
fn test_example_one() {
	let (node_count, edge_count, nodes) = build_graph_from_file("test_cases/example_1.txt");
	let mut dijkstra = Dijkstra::new(&nodes);
	dijkstra.compute_shortest_paths(0);
//	assert_eq!(dijkstra.path_solutions()[0], Reach(0));
//	assert_eq!(dijkstra.path_solutions()[1], Reach(-5));
//	assert_eq!(dijkstra.path_solutions()[2], Reach(-4));
//	assert_eq!(dijkstra.path_solutions()[3], Reach(-3));
//	assert_eq!(dijkstra.path_solutions()[4], Reach(-10003));
//	assert_eq!(dijkstra.path_solutions()[5], Reach(-10));
}
