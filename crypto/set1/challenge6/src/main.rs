use base64;

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

//calculate hamming distance between strings
// fn calc_ham_dist(s1: &str, s2: &str) -> i32{
// 	if s1.len() != s2.len() {
// 		return 0;
// 	}

// 	let s1_bytes = s1.as_bytes().to_vec();
// 	let s2_bytes = s2.as_bytes().to_vec();
// 	let mut hamming_dist: i32 = 0;

// 	println!("{:?}", s1_bytes);

// 	for (byte1, byte2) in s1_bytes.iter().zip(s2_bytes.iter()) {
// 		for x in 0..8 {
// 			// println!("val1: {:?} val2: {:?}", (byte1 & BIT_MASKS[x]) >> x, (byte2 & BIT_MASKS[x]) >> x);
// 			if ((byte1 & BIT_MASKS[x]) >> x) != ((byte2 & BIT_MASKS[x]) >> x) {
// 				hamming_dist += 1;
// 			}
// 		}
// 	}

// 	return hamming_dist;
// }


fn calc_ham_dist(s1: Vec<u8>, s2: Vec<u8>) -> i32{
	if s1.len() != s2.len() {
		return 0;
	}

	let mut hamming_dist: i32 = 0;

	for (byte1, byte2) in s1.iter().zip(s2.iter()) {
		for x in 0..8 {
			// println!("val1: {:?} val2: {:?}", (byte1 & BIT_MASKS[x]) >> x, (byte2 & BIT_MASKS[x]) >> x);
			if ((byte1 & BIT_MASKS[x]) >> x) != ((byte2 & BIT_MASKS[x]) >> x) {
				hamming_dist += 1;
			}
		}
	}

	return hamming_dist;
}

fn find_key_size(input: &str) -> i32 {
	let input_bytes = base64::decode(&input).unwrap().to_vec();

	// println!("input bytes: {:?} ", input_bytes);

	for x in 2..10 {
		println!("{:?}", calc_ham_dist(input_bytes[0..x].to_vec(), input_bytes[x..2*x].to_vec()));
	}

	return 0;
}


//need to find a way to input strings as one line instead of separate lines.. newline character causing error in decoding
fn main() {
	// let s1 = "this is a test";
	// let s2 = "wokka wokka!!!";

	//60 base64 chars per line => 60 chars / 4 chars/3 bytes = 45 bytes
	let test_str = "HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS";

	find_key_size(&test_str);

}
