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

pub struct Solver {
	pub items: Vec<Item>,
	memo: Vec<Memo>,
}

impl Solver {
	pub fn new(items: Vec<Item>) -> Solver {
		let mut memo = Solver::create_memo(&items);
		Solver { items: items, memo: memo }
	}

	pub fn solve(&mut self, knapsack_size: u32) {
		println!("GCD is {}", self.greatest_common_divisor());
	}

	fn create_memo(items: &Vec<Item>) -> Vec<Memo> {
		let mut memo = Vec::<Memo>::with_capacity(items.len());
		for m in memo.iter_mut() {
			m.optimal_value = 0;
		}
		memo
	}

	// Greatest common divisor among the weights
	// All examples have a GCD of 1, so commented out because has no use.
//	fn greatest_common_divisor(&self) -> u32 {
//		let mut primes = [2,3,5,7,11,13,17];
//		primes.reverse();
//
//		for prime in primes.iter() {
//			let all_divisible = self.items.iter().all(|item| { item.weight % prime == 0 });
//			if (all_divisible) {
//				return *prime as u32;
//			}
//		}
//
//		1
//	}
}
