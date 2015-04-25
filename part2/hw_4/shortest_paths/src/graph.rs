use std::ops::{Add, Sub};
use graph::PathLength::{Reach, Unreach};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy)]
pub struct DirectedEdge {
	pub weight: i32,
	pub a: usize,
	pub b: usize,
}

impl DirectedEdge {
	pub fn new(a: usize, b: usize, weight: i32) -> DirectedEdge {
		DirectedEdge { weight: weight, a: a, b: b }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DijkstraEdge {
	pub path_length: i32,
	pub a: usize,
	pub b: usize,
}

impl DijkstraEdge {
	pub fn new(length_to_edge_source: i32, edge: &DirectedEdge) -> DijkstraEdge {
		DijkstraEdge { path_length: length_to_edge_source + edge.weight, a: edge.a, b: edge.b }
	}
}

// Note that this is implemented in reverse path length order so that comparing a <=> b,
// a is less than b when a's path length is greater than b's so that DirectedEdges can
// be used in a max-heap and be withdrawn in order from lowest to highest
impl Ord for DijkstraEdge {
	fn cmp(&self, other: &Self) -> Ordering {
		other.path_length.cmp(&self.path_length)
	}
}

impl PartialOrd for DijkstraEdge {
	fn partial_cmp(&self, other: &DijkstraEdge) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub struct NodeBuilder {
	pub index: usize,
	pub outgoing_edge_count: usize,
	pub incoming_edge_count: usize,
}

impl NodeBuilder {
	pub fn new(index: usize) -> NodeBuilder {
		NodeBuilder { index: index, outgoing_edge_count: 0, incoming_edge_count: 0 }
	}

	pub fn to_node(&self) -> Node {
		Node::new(self)
	}
}

#[derive(Debug)]
pub struct Node {
	pub index: usize,
	pub out_edges: Vec<DirectedEdge>,
	pub in_edges: Vec<DirectedEdge>,
}

impl Node {
	pub fn new(builder: &NodeBuilder) -> Node {
		Node { index: builder.index,
			out_edges: Vec::<DirectedEdge>::with_capacity(builder.outgoing_edge_count),
			in_edges: Vec::<DirectedEdge>::with_capacity(builder.incoming_edge_count)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum PathLength {
	Reach(i32),
	Unreach
}

impl PathLength {
	pub fn cat(self, edge: &DirectedEdge) -> PathLength {
		match self {
			PathLength::Reach(self_value) => PathLength::Reach(self_value + edge.weight),
			PathLength::Unreach => PathLength::Unreach,
		}
	}
}

impl Ord for PathLength {
	fn cmp(&self, other: &Self) -> Ordering {
		match (*self, *other) {
			(Reach(self_value), Reach(other_value)) => self_value.cmp(&other_value),
			(Unreach, Unreach) => Ordering::Equal,
			(Unreach, _) => Ordering::Greater,
			(_, Unreach) => Ordering::Less,
		}
	}
}

impl Add for PathLength {
	type Output = PathLength;

	fn add(self, _rhs: PathLength) -> PathLength {
		match (self, _rhs) {
			(PathLength::Reach(self_value), PathLength::Reach(rhs_value)) => PathLength::Reach(self_value + rhs_value),
			(PathLength::Unreach, _) => PathLength::Unreach,
			(_, PathLength::Unreach) => PathLength::Unreach,
		}
	}
}

impl Sub for PathLength {
	type Output = PathLength;

	fn sub(self, _rhs: PathLength) -> PathLength {
		match (self, _rhs) {
			(PathLength::Reach(self_value), PathLength::Reach(rhs_value)) => PathLength::Reach(self_value - rhs_value),
			(PathLength::Unreach, _) => PathLength::Unreach,
			(_, PathLength::Unreach) => PathLength::Unreach,
		}
	}
}

impl PathLength {
	pub fn length(&self) -> i32 {
		match *self {
			Reach(self_value) => self_value,
			Unreach => panic!("Cannot take length of unreachable path")
		}
	}
}

#[test]
fn test_path_add_path() {
	assert_eq!(Reach(3) + Reach(2), Reach(5));
	assert_eq!(Reach(3) + Unreach, Unreach);
	assert_eq!(Unreach + Reach(3), Unreach);
	assert_eq!(Reach(-3) + Reach(3), Reach(0));
}

#[test]
fn test_path_cat_edge() {
	let de_minus_one = DirectedEdge { weight: -1, a: 0, b: 1 };
	let de_plus_one = DirectedEdge { weight: 1, a: 0, b: 1 };

	assert_eq!(Reach(3).cat(&de_minus_one), Reach(2));
	assert_eq!(Reach(3).cat(&de_plus_one), Reach(4));
	assert_eq!(Unreach.cat(&de_plus_one), Unreach);
}

#[test]
fn test_path_comparison() {
	assert_eq!(Reach(3).cmp(&Reach(4)), Ordering::Less);
	assert_eq!(Reach(5).cmp(&Reach(4)), Ordering::Greater);
	assert_eq!(Reach(3).cmp(&Reach(3)), Ordering::Equal);
	assert_eq!(Reach(3).cmp(&Unreach), Ordering::Less);
	assert_eq!(Unreach.cmp(&Reach(3)), Ordering::Greater);
	assert_eq!(Unreach.cmp(&Unreach), Ordering::Equal);
}

#[test]
fn test_edge_comparison() {
	let e0 = DijkstraEdge { path_length: 0, a: 0, b: 1 };
	let e1 = DijkstraEdge { path_length: 1, a: 0, b: 1 };
	let e2 = DijkstraEdge { path_length: 2, a: 0, b: 1 };

	assert_eq!(e0.cmp(&e0), Ordering::Equal);
	assert_eq!(e0.cmp(&e1), Ordering::Greater);
	assert_eq!(e2.cmp(&e1), Ordering::Less);

	let mut heap = BinaryHeap::new();
	heap.push(e0);
	heap.push(e1);
	heap.push(e2);

	assert_eq!(heap.pop().unwrap(), e0);
	assert_eq!(heap.pop().unwrap(), e1);
	assert_eq!(heap.pop().unwrap(), e2);
}

#[test]
fn test_path_length_length() {
	assert_eq!(Reach(5).length(), 5);
}

#[test]
#[should_panic]
fn test_path_length_unreach() {
	Unreach.length();
}