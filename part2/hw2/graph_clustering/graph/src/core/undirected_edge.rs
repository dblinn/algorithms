use core::node::Node;

#[derive(Debug, Clone)]
pub struct UndirectedEdge {
	pub weight: i32,
	pub a: u32,
	pub b: u32,
}

impl UndirectedEdge {
	pub fn new(weight: i32, a: u32, b: u32) -> UndirectedEdge {
		UndirectedEdge { weight: weight, a: a, b: b}
	}

	pub fn connects_to(&self, node: &Node) -> bool {
		self.a == node.index as u32 || self.b == node.index as u32
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