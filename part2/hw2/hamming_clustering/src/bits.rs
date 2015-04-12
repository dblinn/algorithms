pub struct Bits;

impl Bits {
	// Counting bits set, Brian Kernighan's way
	// Rust doesn't like the faster method recommended at
	// https://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetParallel
	// because it takes advantage of arithmetic overflow
	pub fn count(number: &u32) -> u32 {
		let mut v = *number; // count the number of bits set in v
		let mut c = 0; // c accumulates the total bits set in v
		loop
		{
			v &= v - 1; // clear the least significant bit set
			c = c + 1;
			if v == 0 { break; }
		}
		c
	}

	// Calculate the hamming distance
	pub fn hamming_distance(a: &u32, b: &u32) -> u32 {
		0
	}
}

#[test]
fn test_bit_count() {
	assert_eq!(Bits::count(& 1), 1);
	assert_eq!(Bits::count(& 2), 1);
	assert_eq!(Bits::count(& 3), 2);
	assert_eq!(Bits::count(& 0x0000000F), 4);
	assert_eq!(Bits::count(& 0xF000000F), 8);
	assert_eq!(Bits::count(& 0xF000000F), 8);
	assert_eq!(Bits::count(& 0b0100010110011), 6);
}

#[test]
fn test_hamming_distance() {

}