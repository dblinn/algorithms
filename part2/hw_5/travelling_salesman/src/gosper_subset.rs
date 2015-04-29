/**
 * Implementation of Gosper's hack to choose subsets of a given size
 * Iterate using a custom iterator
 */

use std::mem::size_of;
use std::cmp;

// TODO: make generic, allow u32 or u64, follow the PhantomData example
// http://rustbyexample.com/generics/phantom.html
pub struct Gosper {
	max_set_value: u32,
	subset_size: usize,
	curr: u32,
	next: u32
}

impl Iterator for Gosper {
	type Item = u32;

	// The 'Iterator' trait only requires the 'next' method to be defined. The
	// return type is 'Option<T>', 'None' is returned when the 'Iterator' is
	// over, otherwise the next value is returned wrapped in 'Some'
	fn next(&mut self) -> Option<u32> {
		if self.curr >= self.max_set_value { return None; }

		self.curr = self.next;
		self.next = self.next_gosper(self.next);

		Some(self.curr)
	}
}

impl Gosper {
	pub fn new(subset_size: usize, set_size: usize) -> Gosper {
		let max_set_size = Gosper::max_set_size();
		if set_size > max_set_size { panic!("Cannot create set of size {}. Does not currently support sets of size greater than {}.", set_size, max_set_size); }
		if subset_size > set_size { panic!("Cannot choose {} element subsets from an overall set of size {}.", subset_size, set_size); }

		let initial_value = Gosper::initial_value(subset_size);
		// Need to sub 1 from max_set_size in case set_size is 0, cannot left shift 32 bits for 32 bit value, get overflow error
		let max_value = initial_value << cmp::min(set_size - subset_size, max_set_size - 1);
		Gosper { max_set_value: max_value, subset_size: subset_size, curr: 0, next: initial_value }
	}

	pub fn max_set_size() -> usize {
		size_of::<u32>() * 8
	}

	// Counting bits set, Brian Kernighan's way
	pub fn set_size(&self) -> usize {
		let mut v = self.max_set_value; // count the number of bits set in v
		let mut c = 0; // c accumulates the total bits set in v
		while v > 0 {
			v &= v - 1; // clear the least significant bit set
			c = c + 1;
		}
		c
	}

	pub fn subset_size(&self) -> usize { self.subset_size }

	// http://read.seas.harvard.edu/cs207/2012/?p=64
	// Cannot use -x, have to use !x + 1 instead because unary '-' generates an error for unsigned int
	// have to use wrapping add because of rust's overflow arithmetic rules
	fn next_gosper(&self, x: u32) -> u32 {
		let y = x & (!x + 1);
		let c = x.wrapping_add(y);
		(((x ^ c) >> 2) / y) | c
	}

	fn initial_value(set_size: usize) -> u32 {
		let mut init_value = 0;
		for _ in 0 .. set_size {
			init_value = (init_value << 1) + 1;
		}
		init_value
	}
}

#[test]
fn test_initial_value() {
	assert_eq!(1, Gosper::new(1, 24).next().unwrap());
	assert_eq!(0b111, Gosper::new(3, 24).next().unwrap());
	assert_eq!(None, Gosper::new(0, 24).next());
	assert_eq!(!(0 as u32), Gosper::new(32, 32).next().unwrap());
}

#[test]
#[should_panic]
fn test_invalid_args() {
	Gosper::new(10, 5);
}

fn test_correctness() {
	let mut gosper = Gosper::new(1, 8);
	let mut combined = 0;
	for set in gosper { combined |= set; }
	assert_eq!(0xFF, combined);

	assert_eq!(vec![0b1, 0b10, 0b100, 0b1000, 0b10000, 0b100000, 0b1000000, 0b10000000],
		Gosper::new(1, 8).collect::<Vec<u32>>());
}