use std::cmp;
use super::bits::Bits;

pub struct BucketItem {
	pub bucket_value: u32,
	pub range_start: usize,
	pub range_end: usize,
	pub sub_buckets: Option<Buckets>,
}

impl BucketItem {
	pub fn new(bucket_value: u32) -> BucketItem {
		BucketItem { bucket_value: bucket_value, range_start: 0, range_end: 0, sub_buckets: None }
	}

	pub fn bucket_count(&self) -> usize {
		self.range_end - self.range_start
	}

	pub fn has_items(&self) -> bool {
		(self.range_end - self.range_start) > 0
	}

	pub fn to_range(&self) -> (usize, usize) {
		(self.range_start, self.range_end)
	}

	pub fn add_ranges(&self, range_vector: &mut Vec<(usize, usize)>, node: u32) {
		match self.sub_buckets {
			Some(ref buckets) => { buckets.associated_ranges(range_vector, node) },
			None => { range_vector.push(self.to_range()) }
		}
	}

	pub fn sub_bucket(&mut self, nodes: &mut Vec<u32>, hash: Box<Fn(&u32) -> u32>, expected_buckets: u32)  {
		if !self.has_items() { return; }
		self.sub_buckets = Some(Buckets::new(nodes, hash, self.range_start, self.range_end, expected_buckets));
	}
}

pub struct Buckets {
	pub ranges: Vec<BucketItem>,
	pub bucket_hash: Box<Fn(&u32) -> u32>,
	pub range_start: usize,
	pub range_end: usize,
}

impl Buckets {
	pub fn new(nodes: &mut Vec<u32>, hash: Box<Fn(&u32) -> u32>, range_start: usize, range_end: usize, expected_buckets: u32) -> Buckets
	{
		let mut items = Vec::with_capacity((expected_buckets + 1) as usize);
		for i in 0..(expected_buckets+1) {
			items.push(BucketItem::new(i));
		}

		// Sort nodes in the range according to the hash function
		nodes[range_start .. range_end].sort_by(|a,b| { hash(a).partial_cmp(&hash(b)).unwrap() });

		let mut buckets = Buckets
		{
			ranges: items,
			bucket_hash: hash,
			range_start: range_start,
			range_end: range_end,
		};
		buckets.generate_items(nodes);
		buckets
	}

	pub fn associated_ranges(&self, range_vector: &mut Vec<(usize, usize)>, node: u32) {
		let bucket_value = (*self.bucket_hash)(&node);
		let bottom = if bucket_value >= 2 { bucket_value - 2 } else { 0 } as usize; // Need this to prevent underflow
		let top = cmp::min((bucket_value + 2) as usize, self.ranges.len() - 1);

		for range in self.ranges[bottom .. top + 1].iter() {
			if range.has_items() {
				range.add_ranges(range_vector, node);
//				range_vector.push(range.to_range());
			}
		}
	}

	// Generate items by iterating each node in the range and every time the hash value changes,
	// start a new bucket and update the range.
	fn generate_items(&mut self, nodes: &Vec<u32>) {
		let mut current_hash = 0;
		let mut range_start = self.range_start;
		let mut i = range_start;

		for node in nodes[self.range_start .. self.range_end].iter() {
			let next_hash = (*self.bucket_hash)(node);
			if current_hash != next_hash {
				self.ranges[current_hash as usize] =
					BucketItem { bucket_value: current_hash, range_start: range_start, range_end: i, sub_buckets: None};
//				println!("{}->{} {}->{}", range_start, i, current_hash, next_hash);
				current_hash = next_hash;
				range_start = i;
			}

			i += 1;
		}

		if range_start < self.range_end {
			self.ranges[current_hash as usize] =
				BucketItem { bucket_value: current_hash, range_start: range_start, range_end: self.range_end, sub_buckets: None};
		}
	}

	pub fn print_contents(&self) {
		for range in self.ranges.iter() {
			println!("{}: {}", range.bucket_value, range.bucket_count());
		}
	}
}

#[test]
fn test_bucket_creation() {
	let mut nodes = vec![1,2,4,8,16,32];
	let mut node_count = nodes.len();
	let mut buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x)}), 0, node_count, 24 );

	assert_eq!(buckets.ranges[0].bucket_count(), 0);
	assert_eq!(buckets.ranges[1].bucket_count(), nodes.len());
	assert_eq!(buckets.ranges[2].bucket_count(), 0);

	nodes = vec![0,0,1,3];
	node_count = nodes.len();
	buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x)}), 0, node_count, 24 );
	assert_eq!(buckets.ranges[0].bucket_count(), 2);
	assert_eq!(buckets.ranges[1].bucket_count(), 1);
	assert_eq!(buckets.ranges[2].bucket_count(), 1);
	assert_eq!(buckets.ranges[3].bucket_count(), 0);


	nodes = vec![0,0];
	node_count = nodes.len();
	buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x)}), 0, node_count, 24 );
	assert_eq!(buckets.ranges[0].bucket_count(), 2);
	assert_eq!(buckets.ranges[1].bucket_count(), 0);

	nodes = vec![];
	node_count = nodes.len();
	buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x)}), 0, node_count, 24 );
	assert_eq!(buckets.ranges[0].bucket_count(), 0);

	nodes = vec![0x00FFFFFF];
	node_count = nodes.len();
	buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x)}), 0, node_count, 24 );
	assert_eq!(buckets.ranges[0].bucket_count(), 0);
	assert_eq!(buckets.ranges[24].bucket_count(), 1);
}

#[test]
fn test_associated_ranges() {
	let mut nodes = vec![];
	let mut x = 0;
	for i in 0..25 {
		nodes.push(x);
		x <<= 1;
		x |= 1;
	}

	let buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x) }), 0, 25, 24);
	let mut ranges = vec![];
	ranges.clear();
	buckets.associated_ranges(&mut ranges, 0);
	assert_eq!(ranges, vec![(0,1),(1,2),(2,3)]);

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 1);
	assert_eq!(ranges, vec![(0,1),(1,2),(2,3),(3,4)]);

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 0b11);
	assert_eq!(ranges, vec![(0,1),(1,2),(2,3),(3,4),(4,5)]);

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 0b111);
	assert_eq!(ranges, vec![(1,2),(2,3),(3,4),(4,5),(5,6)]);

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 0xFFFFFF);
	assert_eq!(ranges, vec![(22,23),(23,24),(24,25)]);
}

#[test]
fn test_simple_sub_buckets() {
	let mut nodes = vec![];
	let mut x = 0;
	nodes.push(0b010000000000000001);

	let node_count = nodes.len();
	let mut buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x) }), 0, node_count, 24);
	for ref mut item in buckets.ranges.iter_mut() {
		item.sub_bucket(&mut nodes, Box::new(|x| { 12 + Bits::count(&(x & 0x00000FFF)) - Bits::count(&(x & 0x00FFF000)) }), 24);
	}

	let mut ranges = vec![];

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 0b1111);
	assert_eq!(ranges, vec![]);

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 0b11);
	assert_eq!(ranges, vec![(0,1)]);
}

#[test]
fn test_complex_sub_buckets() {
	let mut nodes = vec![];
	let mut x = 0;
	for i in 0..25 {
		nodes.push(x);
		x <<= 1;
		x |= 1;
	}

	let node_count = nodes.len();
	let mut buckets = Buckets::new(&mut nodes, Box::new(|x| {Bits::count(x) }), 0, node_count, 24);
	for ref mut item in buckets.ranges.iter_mut() {
		item.sub_bucket(&mut nodes, Box::new(|x| { 12 + Bits::count(&(x & 0x00000FFF)) - Bits::count(&(x & 0x00FFF000)) }), 24);
	}

	let mut ranges = vec![];

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 0);
	assert_eq!(ranges, vec![(0,1),(1,2),(2,3)]);

	ranges.clear();
	buckets.associated_ranges(&mut ranges, 1);
	assert_eq!(ranges, vec![(0,1),(1,2),(2,3),(3,4)]);
}