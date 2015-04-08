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

	pub fn connects_to(&self) -> bool {
		self.a == node.index || self.b == node.index
	}
}
