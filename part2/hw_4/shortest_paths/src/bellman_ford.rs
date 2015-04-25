use std::cmp::Ordering;
use super::graph::*;
use super::graph::PathLength::*;
use super::graph_builder::*;

pub struct BellmanFord {
	pub has_negative_cycle: bool,
	pub first_paths: Vec<PathLength>,
	pub second_paths: Vec<PathLength>,
	pub node_count: usize
}

impl BellmanFord {
	pub fn new(node_count: usize) -> BellmanFord {
		let first_paths = Vec::<PathLength>::with_capacity(node_count);
		let second_paths = first_paths.clone();
		BellmanFord { has_negative_cycle: false, node_count: node_count, first_paths: first_paths, second_paths: second_paths }
	}

	// Returns true if successfully computed paths, returns false if the input graph has a negative cycle
	pub fn compute_shortest_paths(&mut self, source_index: Option<usize>, nodes: &Vec<Node>) -> bool {
		self.init_path_lengths(source_index);

		for i in 0..self.node_count {
			let (source, target) = match i % 2 {
				0 => (&self.first_paths, &mut self.second_paths),
				_ => (&self.second_paths, &mut self.first_paths)
			};

			// The input graph has a negative cycle if the final iteration caused a change
			// to the solution
			self.has_negative_cycle = BellmanFord::step_solution(nodes, source, target);
		}

		!self.has_negative_cycle
	}

	pub fn path_solutions(&self) -> &Vec<PathLength> {
		match self.node_count % 2 {
			0 => &self.first_paths,
			_ => &self.second_paths
		}
	}

	fn init_path_lengths(&mut self, source_index: Option<usize>) {
		let (index, initial_value) = match source_index {
			Some(ndx) => (ndx, Unreach),
			None => (0, Reach(0))
		};

		// Initialize all nodes unreachable except for the source
		for i in 0 .. self.node_count {
			self.first_paths.push(
				if i == index { Reach(0) }
				else { initial_value }
			);
			self.second_paths.push(Unreach);
		}
	}

	// Returns true if any shortest paths were changed by stepping the solution
	fn step_solution(nodes: &Vec<Node>, source: &Vec<PathLength>, target: &mut Vec<PathLength>) -> bool {
		let mut any_changed = false;
		for node in nodes.iter() {
			let next_iteration_shortest_path = node.in_edges.iter()
				.map(|edge| { source[edge.a].cat(edge) })
				.min().unwrap_or(Unreach);
			let previous_iteration_shortest_path = source[node.index];
			target[node.index] = match next_iteration_shortest_path.cmp(&previous_iteration_shortest_path) {
				Ordering::Less => { any_changed = true; next_iteration_shortest_path },
				_ => previous_iteration_shortest_path,
			};
		}

		any_changed
	}
}

#[test]
fn test_example_one() {
	let (node_count, edge_count, nodes) = build_graph_from_file("test_cases/example_1.txt");
	let mut bellman_ford = BellmanFord::new(node_count);
	bellman_ford.compute_shortest_paths(Some(0), &nodes);
	assert!(!bellman_ford.has_negative_cycle);
	assert_eq!(bellman_ford.path_solutions()[0], Reach(0));
	assert_eq!(bellman_ford.path_solutions()[1], Reach(-5));
	assert_eq!(bellman_ford.path_solutions()[2], Reach(-4));
	assert_eq!(bellman_ford.path_solutions()[3], Reach(-3));
	assert_eq!(bellman_ford.path_solutions()[4], Reach(-10003));
	assert_eq!(bellman_ford.path_solutions()[5], Reach(-10));
}

#[test]
fn test_exmaple_two() {
	let (node_count, edge_count, nodes) = build_graph_from_file("test_cases/example_2.txt");
	let mut bellman_ford = BellmanFord::new(node_count);
	bellman_ford.compute_shortest_paths(Some(0), &nodes);
	assert!(!bellman_ford.has_negative_cycle);
	assert_eq!(bellman_ford.path_solutions()[0], Reach(0));
	assert_eq!(bellman_ford.path_solutions()[1], Reach(-2));
	assert_eq!(bellman_ford.path_solutions()[2], Reach(-3));
	assert_eq!(bellman_ford.path_solutions()[3], Reach(-1));
	assert_eq!(bellman_ford.path_solutions()[4], Reach(-6));
	assert_eq!(bellman_ford.path_solutions()[5], Unreach);
}

#[test]
fn test_start_at_zero() {
	let (node_count, edge_count, nodes) = build_graph_from_file("test_cases/example_2.txt");
	let mut bellman_ford = BellmanFord::new(node_count);
	bellman_ford.compute_shortest_paths(None, &nodes);
	assert!(!bellman_ford.has_negative_cycle);
	assert_eq!(bellman_ford.path_solutions()[0], Reach(0));
	assert_eq!(bellman_ford.path_solutions()[1], Reach(-2));
	assert_eq!(bellman_ford.path_solutions()[2], Reach(-3));
	assert_eq!(bellman_ford.path_solutions()[3], Reach(-1));
	assert_eq!(bellman_ford.path_solutions()[4], Reach(-6));
	assert_eq!(bellman_ford.path_solutions()[5], Reach(0));
}

#[test]
fn test_negative_cycle() {
	let (node_count, edge_count, nodes) = build_graph_from_file("test_cases/negative_cycle.txt");
	let mut bellman_ford = BellmanFord::new(node_count);
	bellman_ford.compute_shortest_paths(Some(0), &nodes);
	println!("{:?}", bellman_ford.path_solutions());
	assert!(bellman_ford.has_negative_cycle);
}