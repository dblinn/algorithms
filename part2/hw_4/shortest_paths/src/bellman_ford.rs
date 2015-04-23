use std::cmp::Ordering;
use super::graph::*;
use super::graph::PathLength::*;

pub struct Runner {
	pub has_negative_cycle: bool,
	pub first_paths: Vec<PathLength>,
	pub second_paths: Vec<PathLength>,
	pub node_count: usize
}

impl Runner {
	pub fn new(node_count: usize) -> Runner {
		let first_paths = Vec::<PathLength>::with_capacity(node_count);
		let second_paths = first_paths.clone();
		Runner { has_negative_cycle: false, node_count: node_count, first_paths: first_paths, second_paths: second_paths }
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
			self.has_negative_cycle = Runner::step_solution(nodes, source, target);
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
				if (i == index) { Reach(0) }
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
			target[node.index] = match previous_iteration_shortest_path.cmp(&next_iteration_shortest_path) {
				Ordering::Less => { any_changed = true; previous_iteration_shortest_path },
				_ => next_iteration_shortest_path,
			};
		}

		any_changed
	}
}