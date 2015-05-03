use super::graph::*;
use super::gosper_subset::*;
use std::cmp;
use std::f32;

pub struct Solver {
	pub problem_size: usize,
	pub initial_edges: Vec<SalesmanEdge>,
	pub salesman_edges: Vec<Vec<SalesmanEdge>>,

	choice_size: usize,
	memo: Vec<f32>
}

impl Solver {
	pub fn new(problem_size: usize, initial_edges: Vec<SalesmanEdge>,
			   salesman_edges: Vec<Vec<SalesmanEdge>>) -> Solver
	{
		let memo = Solver::build_memo(problem_size);
		Solver { problem_size: problem_size,
			initial_edges: initial_edges,
			salesman_edges: salesman_edges,
			choice_size: (1 << problem_size),
			memo: memo
		}
	}

	fn build_memo(problem_size: usize) -> Vec<f32> {
		let memo_size = (1 << problem_size) * problem_size;
		vec![0f32; memo_size]
	}

	pub fn solve(&mut self) -> f32 {
		self.initialize_memo();

		for m in 0 .. self.problem_size {
			let mut gosper = Gosper::new(m, self.problem_size);
			for s in gosper {
				for v in 0 .. self.problem_size {
					let s_prime = Solver::masked_subset(s, v as u32);
					let mut subset = BitSubset::new(s_prime);
					let ref edges = self.salesman_edges[v];
					let offset = (s_prime as usize) * self.problem_size;

					// Calculate min over all edges from s_prime to v
					let mut min_cost = f32::MAX;
					for prior_node in subset {
						min_cost = cmp::partial_min(self.memo[offset + prior_node] + edges[prior_node].weight, min_cost).unwrap();
					}
					self.memo[(s as usize) * self.problem_size + v] = min_cost;
				}
			}
		}

		self.calculate_last_leg()
	}

	// Calculate the cost of going back to the start of the loop
	fn calculate_last_leg(&mut self) -> f32 {
		let offset = self.memo.len() - self.problem_size;
		let mut min_cost = f32::MAX;
		let ref edges = self.initial_edges;

		for v in 0 .. self.problem_size {
			min_cost = cmp::partial_min(self.memo[offset + v] + edges[v].weight, min_cost).unwrap();
		}

		min_cost
	}

	#[inline]
	fn masked_subset(s: u32, v: u32) -> u32 {
		let mask = (!0) ^ (1 << v);
		s & mask
	}

	fn initialize_memo(&mut self) {
		for i in 0 .. self.problem_size {
			self.memo[i] = self.initial_edges[i].weight;
		}
	}

	#[inline]
	fn memo_index(&self, s: usize, v: usize) -> usize {
		s + v
	}
}