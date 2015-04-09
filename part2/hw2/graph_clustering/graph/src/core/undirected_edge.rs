use core::node::Node;

#[derive(Debug, Clone)]
pub struct UndirectedEdge {
	pub weight: i32,
	pub a: i32,
	pub b: i32,
}

impl UndirectedEdge {
	pub fn new(weight: i32, a: i32, b: i32) -> UndirectedEdge {
		UndirectedEdge { weight: weight, a: a, b: b}
	}

	pub fn connects_to(&self, node: &Node) -> bool {
		self.a == node.index || self.b == node.index
	}
}

#[test]
fn test_connects_to() {
	let edge = UndirectedEdge::new(0, 0, 1);
	let a = Node::new(0);
	let b = Node::new(1);

	assert!(edge.connects_to(&a));
	assert!(edge.connects_to(&b));
}

#[test]
fn test_does_not_connect_to() {
	let edge = UndirectedEdge::new(0, 0, 1);
	let a = Node::new(2);
	let b = Node::new(-1);

	assert!(!edge.connects_to(&a));
	assert!(!edge.connects_to(&b));
}