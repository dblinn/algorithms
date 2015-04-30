use std::num;

#[derive(Debug, Clone, Copy)]
pub struct SalesmanEdge {
	pub weight: f32,
	pub neighbor: usize,
}

impl SalesmanEdge {
	pub fn new(neighbor: usize, weight: f32) -> SalesmanEdge {
		SalesmanEdge { weight: weight, neighbor: neighbor }
	}
}

#[derive(Clone, Copy)]
pub struct SalesmanPoint { x: f32, y: f32 }

impl SalesmanPoint {
	pub fn distance(&self, other: &salesman_point) {
		let x = self.x - other.x;
		let y = self.y - other.y;
		num::sqrt((x * x) + (y * y))
	}
}