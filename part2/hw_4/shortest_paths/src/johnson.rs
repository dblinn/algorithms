use std::cmp;
use super::graph::*;
use super::graph::PathLength::*;
use super::bellman_ford::BellmanFord;
use super::dijkstra::Dijkstra;

pub struct Johnson {
	pub bellman: BellmanFord,
	pub nodes: Vec<Node>,
	pub edge_count: usize,

	pub shortest_shortest_path: PathLength,
}

// Implementation of Johnson's algorithm that uses BellmanFord to calculate an edge relabeling,
// and then iteratively applies Dijkstra on the relabelled graph to arrive at the shortest shortest paths
// where shortest shortest paths is the min over all starting nodes (max path length calculated by dijkstra for that starting node)
impl Johnson {
	pub fn new(edge_count: usize, nodes: Vec<Node>) -> Johnson {
		Johnson {
			bellman: BellmanFord::new(nodes.len()),
			nodes: nodes,
			edge_count: edge_count,
			shortest_shortest_path: Unreach,
		}
	}

	// Run BellmanFord
	// Return (false, 0) if the graph has a negative cycle
	// Relabel edges
	// Loop
	//		Calculate dijkstra paths
	//		Calculate path length after undoing edge relabelling
	//		Save as shortest if best
	pub fn find_shortest_shortest_path(&mut self) -> (bool, PathLength) {
		println!("Running Bellman Ford");
		if !self.bellman.compute_shortest_paths(None, &self.nodes) {
			println!("Found negative cycle");
			return (false, Unreach);
		}

		let bellman_solutions = self.bellman.path_solutions();
		let relabelled_nodes = self.nodes_with_relabelled_edges(bellman_solutions);
		let mut dijkstra = Dijkstra::new(&relabelled_nodes, self.edge_count);
		self.shortest_shortest_path = self.compute_shortest_shortest_paths(&relabelled_nodes,
			bellman_solutions, &mut dijkstra);

		(true, self.shortest_shortest_path)
	}

	pub fn compute_shortest_shortest_paths(&self, relabelled_nodes: &Vec<Node>,
										   bellman_solutions: &Vec<PathLength>, dijkstra: &mut Dijkstra) -> PathLength {
		let mut shortest = Unreach;
		for i in 0 .. relabelled_nodes.len() {
			if i % 50 == 0 {
				println!("Dijkstra run {} current shortest {:?}", i, shortest);
			}

			dijkstra.compute_shortest_paths(i);
			let iteration_longest = self.shortest_path(i, bellman_solutions, &dijkstra.shortest_paths);
			shortest = cmp::min(shortest, iteration_longest);
		}

		shortest
	}

	pub fn nodes_with_relabelled_edges(&self, bellman_solutions: &Vec<PathLength>) -> Vec<Node> {
		println!("Relabelling edges");
		self.nodes.iter().map(|node| {
			Node {
				index: node.index,
				out_edges: node.out_edges.iter().map(|edge| Johnson::relabelled_edge(edge, bellman_solutions)).collect(),
				in_edges: node.in_edges.iter().map(|edge| Johnson::relabelled_edge(edge, bellman_solutions)).collect(),
			}
		}).collect()
	}

	pub fn shortest_path(&self, source_index: usize, bellman_solutions: &Vec<PathLength>,
						dijkstra_solutions: &Vec<PathLength>) -> PathLength {
		let mut shortest_path = Unreach;

		let mut i = 0;
		for path in dijkstra_solutions.iter() {
			let restored_length = Johnson::restored_path_length(source_index, i, bellman_solutions, *path);
			shortest_path = cmp::min(shortest_path, restored_length);
			i = i + 1;
		}

		shortest_path
	}

	#[inline]
	pub fn relabelled_edge(edge: &DirectedEdge, bellman_solutions: &Vec<PathLength>) -> DirectedEdge {
		DirectedEdge {
			weight: edge.weight + bellman_solutions[edge.a].length() - bellman_solutions[edge.b].length(),
			a: edge.a,
			b: edge.b,
		}
	}

	#[inline]
	pub fn restored_path_length(source_index: usize, dest_index: usize, bellman_solutions: &Vec<PathLength>,
								path_length: PathLength) -> PathLength
	{
		path_length - bellman_solutions[source_index] + bellman_solutions[dest_index]
	}
}
