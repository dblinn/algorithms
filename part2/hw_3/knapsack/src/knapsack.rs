use std::cmp::Ordering;
// Algorithm due to Li Xiaosong in this post:
// https://class.coursera.org/algo2-004/forum/thread?thread_id=194

#[derive(Debug)]
pub struct Item {
	pub value: u32,
	pub weight: u32,
}

impl Item {
	pub fn new(weight: u32, value: u32) -> Item {
		Item { weight: weight, value: value }
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Memo {
	pub optimal_value: u32,
	pub used_weight: u32,
}

impl Memo {
	pub fn add(&self, item: &Item) -> Memo {
		Memo { optimal_value: self.optimal_value + item.value, used_weight: self.used_weight + item.weight }
	}

	pub fn makes_weight(&self, allowed_weight: u32) -> bool {
		self.used_weight <= allowed_weight
	}

	pub fn better_than(&self, other: &Memo) -> bool {
		self.optimal_value > other.optimal_value
	}

	pub fn better_value<'a>(first: &'a Memo, second: &'a Memo) -> &'a Memo {
		if first.optimal_value > second.optimal_value { first }
		else { second }
	}
}

pub struct Solver {
	pub items: Vec<Item>,
	pub knapsack_size: u32,
	first_memo: Vec<Memo>,
	second_memo: Vec<Memo>,
}

impl Solver {
	pub fn new(items: Vec<Item>, knapsack_size: u32) -> Solver {
		let memo_size = items.len() + 1;
		let first_memo = Solver::create_memo(memo_size);
		let second_memo = Solver::create_memo(memo_size);
		Solver { items: items, knapsack_size: knapsack_size, first_memo: first_memo, second_memo: second_memo }
	}

	pub fn solve(&mut self) -> u32 {
		self.first_memo.clear();
		self.second_memo.clear();
		self.first_memo.push(Memo { optimal_value: 0, used_weight: 0 });

		for i in 0..self.items.len() {
			let (source_memo, target_memo) = match i % 2 {
				0 => (&self.first_memo, &mut self.second_memo),
				_ => (&self.second_memo, &mut self.first_memo)
			};
			target_memo.clear();
			Solver::step_solution(&self.items[i], source_memo, target_memo, self.knapsack_size);
		}

		self.solution_memo().optimal_value
	}

	fn solution_memo(&self) -> Memo {
		if self.items.len() % 2 == 0 {
			self.first_memo[self.first_memo.len() - 1]
		}
		else {
			self.second_memo[self.second_memo.len() - 1]
		}
	}

	fn update_target(target_memo: &mut Vec<Memo>, last_added: &Memo, to_add: &Memo, knapsack_size: u32) -> Memo {
		if to_add.makes_weight(knapsack_size) && to_add.better_than(last_added) {
			target_memo.push(*to_add);
			return *to_add;
		}
		*last_added
	}

	fn step_solution(cur_item: &Item, source_memo: &Vec<Memo>, target_memo: &mut Vec<Memo>, knapsack_size: u32) {
		let mut i = 0;
		let mut j = 0;

		let mut last_added = source_memo[0];
		target_memo.push(Memo { optimal_value: 0, used_weight: 0 });
		while i < source_memo.len() && j < source_memo.len() {
			let ref source = source_memo[i];
			let source_prime = source_memo[j].add(cur_item);
			match source.used_weight.cmp(&source_prime.used_weight) {
				Ordering::Less => {
					last_added = Solver::update_target(target_memo, &last_added, source, knapsack_size);
					i = i + 1;
				},
				Ordering::Greater => {
					last_added = Solver::update_target(target_memo, &last_added, &source_prime, knapsack_size);
					j = j + 1;
				},
				Ordering::Equal => {
					last_added = Solver::update_target(target_memo, &last_added,
						&Memo::better_value(source, &source_prime), knapsack_size);
					i = i + 1;
					j = j + 1;
				},
			}
		}

		for k in j .. source_memo.len() {
			let ref source = source_memo[k];
			last_added = Solver::update_target(target_memo, &last_added, &source.add(cur_item), knapsack_size);
		}
	}

	fn create_memo(memo_size: usize) -> Vec<Memo> {
		Vec::<Memo>::with_capacity(memo_size)
	}
}
