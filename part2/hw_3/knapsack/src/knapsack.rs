pub struct Item {
	pub weight: u32,
	pub value: u32,
}

impl Item {
	pub fn new(weight: u32, value: u32) -> Item {
		Item { weight: weight, value: value }
	}
}

pub struct Memo {
	pub optimal_value: u32,
}
