pub struct Edge {
	pub weight: i32,
	pub a: i32,
	pub b: i32,
}

pub struct Node {
	pub index: i32,
	pub edges: Box<Vec<Edge>>,
}

//impl Node {
//	pub fn difference(&self) -> i32 {
//		self.weight - self.duration
//	}
//
//	pub fn ratio(&self) -> f32 {
//		(self.weight as f32) / (self.duration as f32)
//	}
//}

pub struct Graph {
	pub nodes: Box<Vec<Node>>
}

//#[test]
//fn test_difference() {
//	assert_eq!((Job {weight: 10, duration: 5}).difference(), 5);
//}
//
//#[test]
//fn test_ratio() {
//	assert_eq!((Job {weight: 10, duration: 5}).ratio(), 2.0 as f32);
//}