pub struct Item {
	pub value: u32,
	pub weight: u32,
}

impl Item {
	pub fn new(weight: u32, value: u32) -> Item {
		Item { weight: weight, value: value }
	}
}

#[derive(Copy)]
pub struct Memo {
	pub optimal_value: u32,
	pub used_weight: u32,
}

impl Memo {
	pub fn Add(&self, item: &Item) -> Memo {
		Memo { optimal_value: self.optimal_value + item.value, used_weight: self.used_weight + item.weight }
	}

	pub fn makes_weight(&self, allowed_weight: u32) -> bool {
		self.used_weight <= allowed_weight
	}

	pub fn better_than(&self, other: &Memo) -> bool {
		self.optimal_value > other.optimal_value
	}
}

pub struct Solver {
	pub items: Vec<Item>,
	pub knapsack_size: u32,
	memo: Vec<Memo>,
}

impl Solver {
	pub fn new(items: Vec<Item>, knapsack_size: u32) -> Solver {
		let mut memo = Solver::create_memo(knapsack_size);
		Solver { items: items, knapsack_size: knapsack_size, memo: memo }
	}

	pub fn solve(&mut self) -> u32 {
		self.reset_memo();

		for item in self.items.iter() {
			self.step_solution(item);
		}

		self.memo[self.memo.len() - 1].optimal_value
	}

	fn step_solution(&mut self, cur_item: &item) {
		let mut working_weight_memo = Memo { optimal_value: 0, used_weight: 0 };
		let mut last_weight_memo = Memo { optimal_value: 0, used_weight: 0 };

		for i in 0..self.memo.len() {
//			let mut ref prior_memo = self.memo[prior_index(i)];
			let mut ref last_item_memo = self.memo[i];
			working_weight_memo = optimal_value(&last_weight_memo, last_item_memo, cur_item);

			// Copy over into prior memo
			last_weight_memo = working_weight_memo;
			last_item_memo = working_weight_memo;
		}
	}

//	fn prior_index(&self, cur_index: usize) {
//		if (cur_index == 0) {
//			self.knapsack_size() - 1
//		}
//		else {
//			cur_index - 1
//		}
//	}

	fn optimal_value(&self, last_weight_memo: &Memo, last_item_memo: &Memo, item: &Item) -> Memo {
		composite_memo = last_item_memo.add(item);
		if composite_memo.makes_weight(knapsack_size) && composite_memo.better_than(last_weight_memo) {
			return composite_memo;
		}
		last_weight_memo
	}

	fn reset_memo(&mut self) {
		for m in self.memo.iter_mut() {
			m.optimal_value = 0;
		}
	}

	fn create_memo(knapsack_size: u32) -> Vec<Memo> {
		let mut memo = Vec::<Memo>::with_capacity(knapsack_size as usize);
		for i in 0..(knapsack_size as usize) {
			memo.push(Memo { optimal_value: 0} );
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
