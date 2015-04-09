#[derive(Debug)]
pub struct Node {
	pub index: i32,
}

impl Node {
	pub fn new(index: i32) -> Node {
		Node { index: index }
	}
}
