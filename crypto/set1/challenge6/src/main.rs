use base64;
use std::fs::File;
use std::fs;
use std::io::{self, prelude::*, BufReader};

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

//use for two values
fn calc_ham_dist(s1: Vec<u8>, s2: Vec<u8>) -> i32{
	if s1.len() != s2.len() {
		return 0;
	}

	let mut hamming_dist: i32 = 0;

	for (byte1, byte2) in s1.iter().zip(s2.iter()) {
		for x in 0..8 {
			if ((byte1 & BIT_MASKS[x]) >> x) != ((byte2 & BIT_MASKS[x]) >> x) {
				hamming_dist += 1;
			}
		}
	}

	return hamming_dist;
}

//edit/ham distance will find keysize because key will be xor'd out of data.. same character sequences between two strings will not add to ham distance which is why you look for smallest ham distance
fn find_key_size(input: &str) -> Vec<i32> {
	let input_bytes_arr = base64::decode(&input).unwrap();
	let mut ret_key_sizes: Vec<i32> = Vec::new(); 
	let mut smallest_ham_dist = 10000;
	// let mut ham_distance: i32 = 0;
	// println!("input bytes: {:?} ", input_bytes);

	for x in 2..40 {
		let mut input_chunks: Vec<&[u8]> = input_bytes_arr.chunks(x).collect();
		let mut ham_distance: i32 = 0;


		//6 permutations for 4 chunks (3 + 2 + 1); hamming distance between same data will be 0 so don't count those
		for y in 0..4 {
			for z in y..4 {
				ham_distance += calc_ham_dist(input_chunks[y].to_vec(), input_chunks[z].to_vec());
			}
		}

		ham_distance = (ham_distance / 6) / x as i32;

		//build vector with smallest key sizes
		if ham_distance <= smallest_ham_dist {
			if ham_distance == smallest_ham_dist {
				ret_key_sizes.push(x as i32);
			} else {
				ret_key_sizes.clear();
				ret_key_sizes.push(x as i32);

			}
			smallest_ham_dist = ham_distance;
		}

	}

	return ret_key_sizes;
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

	return ret_score;
}

fn xor_data(char_data: &Vec<(u8, f32)>, byte_vec: &Vec<u8>) -> u8 {
	//initialize lowest_score as an arbitrarily large num
	let mut lowest_score: f32 = 10000.0;
	let mut ret_char: u8 = b'a';
	let mut cur_score = 0.0;
	let mut cur_char: u8 = 0;
	let mut temp_vec: Vec<u8> = Vec::new();

	for x in (0 as u8)..(255 as u8) {
		temp_vec = byte_vec.iter()
			.map(|&val| val ^ x)
			.collect();

		cur_score = generate_score(&char_data, &temp_vec, byte_vec.len());

		if cur_score < lowest_score {
			lowest_score = cur_score;
			ret_char = x;
		}

	}

	return ret_char;
}

fn decrypt_data(char_data: &Vec<(u8, f32)>, key_sizes: &Vec<i32>, input: &str) -> Vec<String> {
	let input_bytes_arr = base64::decode(&input).unwrap();
	let mut build_str: Vec<char> = Vec::new();
	let mut ret_arr: Vec<String> = Vec::new();

	for key in key_sizes.iter() {
		let input_chunks: Vec<&[u8]> = input_bytes_arr.chunks(*key as usize).collect();

		//need to form X blocks comprised of byte N from each chunk
		for x in 0..*key {
			let mut new_chunk: Vec<u8> = b"".to_vec();

			for chunk in &input_chunks {
				//need to have if condition to make sure the index doesn't go out of index of the chunk
				if x < chunk.len() as i32 {
					new_chunk.push(chunk[x as usize]);
				}
			}
			build_str.push(xor_data(&char_data, &new_chunk) as char);
		}

		ret_arr.push(build_str.iter().collect());
		build_str.clear();
	}

	return ret_arr;
}

fn open_file(file_name: &str) -> io::Result<String>{
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let mut cur_line: String = "".to_string();

	for line in reader.lines() {
		cur_line = format!("{}{}", cur_line, line.unwrap());
	}

	return Ok(cur_line);
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
	//60 base64 chars per line => 60 chars / 4 chars/3 bytes = 45 bytes

	let mut char_data: Vec<(u8, f32)> = Vec::new();
	initialize_data(&mut char_data);

	let file_str = open_file("full_input.txt").unwrap();

	let key_sizes = find_key_size(&file_str);
	println!("possible keys: {:?}", decrypt_data(&char_data, &key_sizes, &file_str));
}
