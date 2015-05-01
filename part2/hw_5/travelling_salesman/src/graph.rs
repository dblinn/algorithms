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
	pub fn distance(&self, other: &SalesmanPoint) -> f32 {
		let x = self.x - other.x;
		let y = self.y - other.y;
		((x * x) + (y * y)).sqrt()
	}
}

#[test]
fn test_salesman_point()
{
	let sp1 = SalesmanPoint{x: 1.0, y: 3.0};
	let sp2 = SalesmanPoint{x: 2.0, y: 3.0};
	let sp3 = SalesmanPoint{x: 2.0, y: 6.0};
	assert_eq!(0f32, sp1.distance(&sp1));
	assert_eq!(sp1.distance(&sp2), sp2.distance(&sp1));

	let sub_val = sp1.distance(&sp3) - 10f32.sqrt();
	assert!(sub_val.abs() < 0.000001f32);
}