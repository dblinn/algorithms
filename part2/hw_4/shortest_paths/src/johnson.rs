use std::cmp::Ordering;
use super::graph::*;
use super::graph::PathLength::*;
use super::graph_builder::*;
use super::bellman_ford::BellmanFord;
use super::dijkstra::Dijkstra;

pub struct Johnson {
	pub bellman: BellmanFord,
	pub nodes: Vec<Node>,
	pub edge_count: usize,

	pub shortest_shortest_path: i32,
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
			shortest_shortest_path: i32::max_value(),
		}
	}

	// Run BellmanFord
	// Return (false, 0) if the graph has a negative cycle
	// Relabel edges
	// Loop
	//		Calculate dijkstra paths
	//		Calculate path length after undoing edge relabelling
	//		Save as shortest if best
	pub fn find_shortest_shortest_path(&mut self) -> (bool, i32) {
		if !self.bellman.compute_shortest_paths(None, &self.nodes) {
			return (false, 0);
		}

		let bellman_solutions = self.bellman.path_solutions();
		let relabelled_nodes = self.nodes_with_relabelled_edges(bellman_solutions);
//		let dijkstra =
//		self.shortest_shortest_path = compute_shortest_shortest_paths(&relabelled_nodes, &dijkstra);

		(true, self.shortest_shortest_path)
	}

	pub fn nodes_with_relabelled_edges(&self, bellman_solutions: &Vec<PathLength>) -> Vec<Node> {
		self.nodes.iter().map(|node| {
			Node {
				index: node.index,
				out_edges: node.out_edges.iter().map(|edge| self.relabelled_edge(edge, bellman_solutions)).collect(),
				in_edges: node.in_edges.iter().map(|edge| self.relabelled_edge(edge, bellman_solutions)).collect(),
			}
		}).collect()
	}

	#[inline]
	pub fn relabelled_edge(&self, edge: &DirectedEdge, bellman_solutions: &Vec<PathLength>) -> DirectedEdge {
		DirectedEdge {
			weight: edge.weight + bellman_solutions[edge.a].length() - bellman_solutions[edge.b].length(),
			a: edge.a,
			b: edge.b,
		}
	}

	#[inline]
	pub fn restored_path_length(bellman_solutions: &Vec<PathLength>,
								source: &Node, dest: &Node, path_length: i32) -> i32
	{
		path_length - bellman_solutions[source.index].length() + bellman_solutions[dest.index].length()
	}
}
