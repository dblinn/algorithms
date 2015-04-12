pub struct Bits;

impl Bits {
	// Counting bits set, Brian Kernighan's way
	// Rust doesn't like the faster method recommended at
	// https://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetParallel
	// because it takes advantage of arithmetic overflow. WrappingOps currently unstable so not using them.
	#[inline]
	pub fn count(number: &u32) -> u32 {
		let mut v = *number; // count the number of bits set in v
		let mut c = 0; // c accumulates the total bits set in v
		while v > 0
		{
			v &= v - 1; // clear the least significant bit set
			c = c + 1;
		}
		c
	}

	// Calculate the hamming distance
	#[inline]
	pub fn hamming_distance(a: &u32, b: &u32) -> u32 {
		Bits::count(&(a ^ b))
	}
}

#[test]
fn test_bit_count() {
	assert_eq!(Bits::count(& 0), 0);
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
	assert_eq!(Bits::hamming_distance(& 1, & 2), 2);
	assert_eq!(Bits::hamming_distance(& 2, & 3), 1);
	assert_eq!(Bits::hamming_distance(& 3, & 0), 2);
	assert_eq!(Bits::hamming_distance(& 0x0000000F, & 0), 4);
	assert_eq!(Bits::hamming_distance(& 0xF000000F, & 0x0000000F), 4);
	assert_eq!(Bits::hamming_distance(& 0x1000000F, & 0xF000000F), 3);
	assert_eq!(Bits::hamming_distance(& 0b0100010110011, & 0xF0000000), 10);
}