pub struct Bits;

impl Bits {
	// Taken from stanford bit hacks page https://graphics.stanford.edu/~seander/bithacks.html
	// See the section "Counting bits set, in parallel", specifically the part that begins
	// "The best method for counting bits in a 32-bit integer v is the following:"
	// This produces integer overflow which causes rust to panic. There's no easy way to turn this off. Instead, you
	// can use the OverflowingOps (http://doc.rust-lang.org/std/num/wrapping/trait.OverflowingOps.html) trait, but
	// this is marked as unstable, so I'll just use the slower kernighan way.
	pub fn count(number: &u32) -> u32 {
		let mut v = *number;
		v = v - ((v >> 1) & 0x55555555u32);                   	  // reuse input as temporary
		v = (v & 0x33333333u32) + ((v >> 2) & 0x33333333u32);     // temp
		((v + (v >> 4) & 0xF0F0F0Fu32) * 0x1010101u32) >> 24      // count
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