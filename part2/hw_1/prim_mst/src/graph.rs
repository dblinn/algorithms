#[derive(Debug, Clone)]
pub struct UndirectedEdge {
	pub weight: i32,
	pub a: i32,
	pub b: i32,
	pub crosses_cut: bool,
}

impl UndirectedEdge {
	pub fn new(weight: i32, a: i32, b: i32) -> UndirectedEdge {
		UndirectedEdge { weight: weight, a: a, b: b, crosses_cut: false}
	}

	pub fn connects_to(&self, node: &Node) -> bool {
		self.a == node.index || self.b == node.index
	}

	pub fn mark_crossing(&mut self) {
		self.crosses_cut = true;
	}
}

#[derive(Debug)]
pub struct NodeNeighbor {
	pub weight: i32,
	pub neighbor: i32,
}

#[derive(Debug)]
pub struct Node {
	pub index: i32,
	pub in_tree: bool,
	pub edges: Box<Vec<NodeNeighbor>>,
}

// Is all this boilerplate really necessary?
impl PartialEq<Node> for Node {
	fn eq(&self, other: &Node) -> bool {
		if self.index != other.index { return false; }
		if self.edges.len() != other.edges.len() { return false; }
		for i in 0..self.edges.len() {
			if self.edges[i].weight != other.edges[i].weight { return false; }
			if self.edges[i].neighbor != other.edges[i].neighbor { return false; }
		}
		true
	}

	fn ne(&self, other: &Node) -> bool {
		!self.eq(other)
	}
}

impl Node {
	pub fn new(index: i32, edges: Box<Vec<NodeNeighbor>>) -> Node {
		Node { index: index, edges: edges, in_tree: false }
	}
}

#[derive(Debug)]
pub struct Graph {
	pub nodes: Box<Vec<Node>>,
	pub tree_weights: Box<Vec<i32>>,
	pub mst_cost: i64,
}

impl Graph {
	pub fn new(nodes: Box<Vec<Node>>) -> Graph {
		Graph { nodes: nodes, tree_weights: Box::new(Vec::new()), mst_cost: 0 }
	}

	pub fn node(&self, node_index: i32) -> &Node {
		&(*self.nodes)[node_index as usize]
	}

	pub fn node_index_not_in_tree(&self, edge: &UndirectedEdge) -> Option<i32> {
		if !self.nodes[edge.a as usize].in_tree { return Some(edge.a); }
		if !self.nodes[edge.b as usize].in_tree { return Some(edge.b); }
		None
	}

	pub fn mark_in_tree(&mut self, node_index: i32, edge_weight: i32) {
		let ref mut node = &mut self.nodes[node_index as usize];
		assert!(!node.in_tree);
		node.in_tree = true;
		self.tree_weights.push(edge_weight);
		self.mst_cost += edge_weight as i64;
	}

	pub fn create_nodes(node_count: i32, edges: &Vec<UndirectedEdge>) -> Vec<Node> {
		let mut nodes: Vec<Node> = Vec::with_capacity(node_count as usize);
		for i in 0..node_count {
			nodes.push(Node::new(i, Box::new(Vec::new())));
		}

		for edge in edges.iter() {
			nodes[edge.a as usize].edges.push(NodeNeighbor { weight: edge.weight, neighbor: edge.b });
			nodes[edge.b as usize].edges.push(NodeNeighbor { weight: edge.weight, neighbor: edge.a });
		}

		nodes
	}
}

#[test]
fn test_connects_to() {
	let edge = UndirectedEdge::new(0, 0, 1);
	let a = Node::new(0, Box::new(vec![]));
	let b = Node::new(1, Box::new(vec![]));

	assert!(edge.connects_to(&a));
	assert!(edge.connects_to(&b));
}

#[test]
fn test_does_not_connect_to() {
	let edge = UndirectedEdge::new(0, 0, 1);
	let a = Node::new(2, Box::new(vec![]));
	let b = Node::new(-1, Box::new(vec![]));

	assert!(!edge.connects_to(&a));
	assert!(!edge.connects_to(&b));
}

#[test]
fn test_create_nodes() {
	let edges = vec![
		UndirectedEdge::new(1, 0, 1),
		UndirectedEdge::new(-1, 0, 2),
		UndirectedEdge::new(2, 2, 1),
	];
	let expected_nodes = vec![
		Node::new(0, Box::new(vec![NodeNeighbor {weight: 1, neighbor: 1}, NodeNeighbor {weight: -1, neighbor: 2}])),
		Node::new(1, Box::new(vec![NodeNeighbor {weight: 1, neighbor: 0}, NodeNeighbor {weight: 2, neighbor: 2}])),
		Node::new(2, Box::new(vec![NodeNeighbor {weight: -1, neighbor: 0}, NodeNeighbor {weight: 2, neighbor: 1}])),
		Node::new(3, Box::new(vec![])),
	];
	let nodes = Graph::create_nodes(4, &edges);
	for i in 0..nodes.len() {
		assert_eq!(nodes[i], expected_nodes[i]);
	}
}

#[test]
fn test_node_index_not_in_tree() {
	let edges = vec![
		UndirectedEdge::new(1, 0, 1),
		UndirectedEdge::new(-1, 0, 2),
		UndirectedEdge::new(2, 2, 1)
	];
	let mut graph = Graph::new(Box::new(Graph::create_nodes(3, &edges)));

	let edge = UndirectedEdge::new(1, 0, 1);
	graph.mark_in_tree(0, 0);
	assert_eq!(graph.node_index_not_in_tree(&edge).unwrap(), 1);
	graph.mark_in_tree(1, 0);
	assert_eq!(graph.node_index_not_in_tree(&edge), None);
}

// ---------------------------------------------------------------------------------------------------------------------

pub trait MstGreedyFinder {
	fn edges(&self) -> &Vec<UndirectedEdge>;
	fn done(&self) -> bool { self.edges().is_empty() }
	fn minimum_edge(&self) -> UndirectedEdge;
	fn greedy_node_index(&self, graph: &Graph) -> (i32, i32);
	fn remove_related_edges(&mut self, node: &Node);
}

pub struct BruteForceMstGreedyFinder {
	pub edges: Box<Vec<UndirectedEdge>>,
}

impl MstGreedyFinder for BruteForceMstGreedyFinder {
	fn edges(&self) -> &Vec<UndirectedEdge> {
		&*(self.edges)
	}

	fn minimum_edge(&self) -> UndirectedEdge {
		let min_edge = (*self.edges).iter().min_by(|edge| edge.weight).unwrap();
		(*min_edge).clone()
	}

	// Find the node in lowest edge node that crosses the cut.
	fn greedy_node_index(&self, graph: &Graph) -> (i32, i32) {
		let min_edge = (*self.edges).iter()
			.filter(|edge| edge.crosses_cut)
			.min_by(|edge| edge.weight)
			.unwrap();
		(
			graph.node_index_not_in_tree(min_edge).unwrap(),
			min_edge.weight
		)
	}

	// Remove all edges already crossing the cut that connect to the input edge
	fn remove_related_edges(&mut self, node: &Node) {
		(*self.edges).retain(|edge| {
			!(edge.crosses_cut && edge.connects_to(node))
		});
		for edge in (*self.edges).iter_mut() { if edge.connects_to(node) { edge.mark_crossing(); } }
	}
}

#[test]
fn it_only_uses_edges_crossing_the_cut() {
	let edges = vec![
		UndirectedEdge { weight: 1, a: 0, b: 1 , crosses_cut: false },
		UndirectedEdge { weight: -1, a: 0, b: 2, crosses_cut: false },
		UndirectedEdge { weight: 2, a: 1, b: 2, crosses_cut: true },
	];
	let finder = BruteForceMstGreedyFinder { edges: Box::new(edges) };

	let graph = Graph::new(Box::new(Graph::create_nodes(3, finder.edges())));
	let (node_index, edge_weight) = finder.greedy_node_index(&graph);
	assert_eq!(node_index, 1);
}

#[test]
// This would be much write and understand if we could mock and double.
fn it_picks_the_min_edge_crossing_the_cut() {
	let edges = vec![
		UndirectedEdge { weight: 2, a: 0, b: 1 , crosses_cut: true },
		UndirectedEdge { weight: 1, a: 1, b: 2, crosses_cut: true },
		UndirectedEdge { weight: -1, a: 2, b: 0, crosses_cut: false },
	];
	let finder = BruteForceMstGreedyFinder { edges: Box::new(edges) };

	let mut graph = Graph::new(Box::new(Graph::create_nodes(3, finder.edges())));
	graph.mark_in_tree(1, 0);
	let (node_index, edge_weight) = finder.greedy_node_index(&graph);
	assert_eq!(node_index, 2);
}

#[test]
fn it_removes_related_edges() {
	let edges = vec![
		UndirectedEdge { weight: 2, a: 0, b: 1 , crosses_cut: true },
		UndirectedEdge { weight: 1, a: 1, b: 2, crosses_cut: true },
		UndirectedEdge { weight: -1, a: 2, b: 0, crosses_cut: false },
	];
	let mut finder = BruteForceMstGreedyFinder { edges: Box::new(edges) };

	let mut graph = Graph::new(Box::new(Graph::create_nodes(3, finder.edges())));
	graph.mark_in_tree(0, 0);
	finder.remove_related_edges(&(Node { index: 1, in_tree: true, edges: Box::new(vec![]) }));
	assert_eq!(finder.edges().len(), 1);
}

#[test]
fn it_marks_remaining_related_edges_as_crossing_the_cut() {
	let edges = vec![
		UndirectedEdge { weight: 2, a: 0, b: 1 , crosses_cut: true },
		UndirectedEdge { weight: 1, a: 0, b: 2, crosses_cut: true },
		UndirectedEdge { weight: -1, a: 2, b: 1, crosses_cut: false },
	];
	let mut finder = BruteForceMstGreedyFinder { edges: Box::new(edges) };

	let mut graph = Graph::new(Box::new(Graph::create_nodes(3, finder.edges())));
	graph.mark_in_tree(0, 0);
	finder.remove_related_edges(&(Node { index: 1, in_tree: true, edges: Box::new(vec![]) }));
	assert!(finder.edges().iter().all(|edge| edge.crosses_cut ));
}