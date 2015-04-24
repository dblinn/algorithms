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

impl Johnson {
	pub fn new(edge_count: usize, nodes: Vec<Node>) -> Johnson {
		Johnson {
			bellman: BellmanFord::new(nodes.len()),
			nodes: nodes,
			edge_count: edge_count,
			shortest_shortest_path: i32::max_value(),
		}
	}
}
