use core::node::Node;

#[derive(Debug)]
pub struct Graph {
	pub nodes: Vec<Node>,
}

impl Graph {
	pub fn new(nodes: Vec<Node>) -> Graph {
		Graph { nodes: nodes }
	}

	pub fn create_nodes(node_count: i32) -> Vec<Node> {
		let mut nodes: Vec<Node> = Vec::with_capacity(node_count as usize);
		for i in 0..node_count {
			nodes.push(Node::new(i));
		}
		nodes
	}
}

#[test]
fn test_create_nodes() {
	let expected_nodes = [
		Node::new(0),
		Node::new(1),
		Node::new(2),
		Node::new(3),
	];
	let nodes = Graph::create_nodes(4);
	for i in 0..nodes.len() {
		assert_eq!(nodes[i], expected_nodes[i]);
	}
}
