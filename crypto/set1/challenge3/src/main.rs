// use hex;
// use std::collections::HashMap;

fn convert_to_u8(hex_string: &str) -> Vec<u8> {
	let mut byte_vec = Vec::new();

	// for i in 0..hex_string.len() {
	for i in (0..hex_string.len()).step_by(2) {

		let decimal_byte = u8::from_str_radix(&hex_string[i..i+2], 16);
		println!("{:?}", decimal_byte);

		match decimal_byte {
			Ok(val) => byte_vec.push(val),
			Err(_) => println!("{:?}", "there was a problem with string size (not divisible)"),
		}
	}

	for x in byte_vec.iter() {
		println!("{:?}", x);
	}

	return byte_vec;
}

// fn xor_strings(s1: &str, s2: &str) -> String {
// 	let s1_from_hex = convert_to_u8(&s1);
// 	let s2_from_hex = convert_to_u8(&s2);	

// 	let xor_vals: Vec<u8> = s1_from_hex.iter()
// 		.zip(s2_from_hex.iter())
// 		.map(|(&x1, &x2)| x1 ^ x2)
// 		.collect();

// 	let ret_str = hex::encode(xor_vals);
// 	println!("{:?}", ret_str);

// 	return ret_str.to_string();
// }

fn generate_score(char_vals: &Vec<(u8, f32)>) -> f32 {
	//compare the score of each letter
	//compute the difference in relative frequency (abs val) --> add it all together
	//the highest score is bad
	return 0.0;
}

fn xor_data(char_vals: Vec<(u8, f32)>) -> String {
	let mut lowest_score: f32 = 0.0;
	let mut ret_str: &str = "";
	let mut cur_score = 0.0;
	ret_str = "test";
	lowest_score = 0.1;

	for x in 0..255 {
		cur_score = generate_score(&char_vals);
		if cur_score < lowest_score {
			// lowest_score = generate_score(char_vals);

			//compress the byte array into string
			// ret_str = data.collect();
		}

	}

	return ret_str.to_string();
}


fn initialize_data(char_vals: &mut Vec<(u8, f32)>) {
	char_vals.push((b'a', 8.5));
	char_vals.push((b'b', 1.5));
	char_vals.push((b'c', 2.2));
	char_vals.push((b'd', 4.3));
	char_vals.push((b'e', 11.1));
	char_vals.push((b'f', 2.2));
	char_vals.push((b'g', 2.0));
	char_vals.push((b'h', 6.0));
	char_vals.push((b'i', 7.5));
	char_vals.push((b'j', 0.2));
	char_vals.push((b'k', 1.3));
	char_vals.push((b'l', 4.0));
	char_vals.push((b'm', 2.4));
	char_vals.push((b'n', 6.7));
	char_vals.push((b'o', 7.5));
	char_vals.push((b'p', 1.9));
	char_vals.push((b'q', 0.1));
	char_vals.push((b'r', 7.6));
	char_vals.push((b's', 6.3));
	char_vals.push((b't', 9.4));
	char_vals.push((b'u', 2.8));
	char_vals.push((b'v', 1.0));
	char_vals.push((b'w', 2.6));
	char_vals.push((b'x', 0.2));
	char_vals.push((b'y', 2.0));
	char_vals.push((b'z', 0.1));
}

fn main() {
	// let mut char_vals:HashMap<u8, f32> = HashMap::new();
	let mut char_data: Vec<(u8, f32)> = Vec::new();
	let byte_vec = convert_to_u8(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	//switch this to hold the values of the current count
	initialize_data(&mut char_data);


	//need to create an array of tuples to hold the char_vals (the ratio for each char) and reference that array when computing the score



	// for x in (0 as u8)..(255 as u8) {
	for x in 0..255 {

		println!("{:?}", x);
		println!("xor val: {:?}", byte_vec[0] ^ x);


	}

	for x in char_data.iter() {
		println!("{:?}", x);
	}
}
