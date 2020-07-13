static BIT_MASKS: [u8; 8] = [
	0x01, //0x01 00000001
	0x02, //0x02 00000010
	0x04, //0x04 00000100
	0x08, //0x08 00001000
	0x10, //0x10 00010000
	0x20, //0x20 00100000
	0x40, //0x40 01000000
	0x80, //0x80 10000000
];

fn main() {
	let mut test = b"aa".to_vec();
	let mut one = 1;

	for byte in test.iter() {
		for x in 0..8 {
			println!("mask: {:?} val: {:?}", BIT_MASKS[x], (byte & BIT_MASKS[x]) >> x);
		}
	}

}
