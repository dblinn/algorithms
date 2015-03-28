pub struct Job {
	pub weight: i32,
	pub duration: i32,
}

impl Job {
	pub fn difference(&self) -> i32 {
		self.weight - self.duration
	}

	pub fn ratio(&self) -> f32 {
		(self.weight as f32) / (self.duration as f32)
	}
}

#[test]
fn test_difference() {
	assert_eq!((Job {weight: 10, duration: 5}).difference(), 5);
}

#[test]
fn test_ratio() {
	assert_eq!((Job {weight: 10, duration: 5}).ratio(), 2.0 as f32);
}