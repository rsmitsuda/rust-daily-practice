use std::str;
// use std::io;
use std::fs::File;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn convert_to_u8(hex_string: &str) -> Vec<u8> {
	let mut byte_vec = Vec::new();

	for i in (0..hex_string.len()).step_by(2) {
		let decimal_byte = u8::from_str_radix(&hex_string[i..i+2], 16);

		match decimal_byte {
			Ok(val) => byte_vec.push(val),
			Err(_) => println!("{:?}", "there was a problem with string size (not divisible)"),
		}
	}

	return byte_vec;
}

fn generate_score(char_vals: &Vec<(u8, f32)>, byte_vec: &Vec<u8>, byte_vec_size: usize) -> f32 {
	//compare the score of each letter
	let mut cur_char_count = 0;
	let mut ret_score: f32 = 0.0;
	// let mut ctr = 1;

	for (key, occurence) in char_vals.iter() {
		cur_char_count = byte_vec.iter()
			.filter(|&x| *x == *key)
			.count();

		ret_score += (((cur_char_count as f32) / (byte_vec_size as f32) * 100.0) - occurence).abs();

	}
	// println!("ret_score: {:?}", ret_score);

	return ret_score;
}

fn xor_data(char_vals: &Vec<(u8, f32)>, byte_vec: &Vec<u8>) -> String {
	//initialize lowest_score as an arbitrarily large num
	let mut lowest_score: f32 = 10000.0;
	let mut ret_str: String = "".to_string();
	let mut cur_score = 0.0;
	let mut cur_char: u8 = 0;
	let mut temp_vec: Vec<u8> = Vec::new();

	for x in (0 as u8)..(255 as u8) {
		temp_vec = byte_vec.iter()
			.map(|&val| val ^ x)
			.collect();

		// println!("the string: {:?}", String::from_utf8_lossy(&temp_vec));
		cur_score = generate_score(&char_vals, &temp_vec, byte_vec.len());

		if cur_score < lowest_score {
			lowest_score = cur_score;
			ret_str = String::from_utf8_lossy(&temp_vec).to_string();
		}

	}

	// println!("FINAL STRING{:?}", ret_str);
	return ret_str;
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

fn open_file(file_name: &str) -> io::Result<()>{
	// let f = fs::read_to_string(file_name)
		// .expect("cannot open file");

	let file = File::open(file_name)?;
	let reader = BufReader::new(file);

	for line in reader.lines() {
		println!("{:?}", line.unwrap());
	}

	Ok(())
}

fn main() {
	let mut char_data: Vec<(u8, f32)> = Vec::new();
	let byte_vec = convert_to_u8(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

	open_file(&"input.txt");

	initialize_data(&mut char_data);
	println!("{:?}", xor_data(&char_data, &byte_vec));

}
