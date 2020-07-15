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
			// println!("val1: {:?} val2: {:?}", (byte1 & BIT_MASKS[x]) >> x, (byte2 & BIT_MASKS[x]) >> x);
			if ((byte1 & BIT_MASKS[x]) >> x) != ((byte2 & BIT_MASKS[x]) >> x) {
				hamming_dist += 1;
			}
		}
	}

	return hamming_dist;
}

fn find_key_size(input: &str) -> Vec<i32> {
	let input_bytes_arr = base64::decode(&input).unwrap();
	let mut ret_key_sizes: Vec<i32> = Vec::new(); 
	let mut smallest_ham_dist = 10000;
	// let mut ham_distance: i32 = 0;
	// println!("input bytes: {:?} ", input_bytes);

	for x in 2..40 {
		let mut input_chunks: Vec<&[u8]> = input_bytes_arr.chunks(x).collect();
		let mut ham_distance: i32 = 0;


		for y in 0..4 {
			for z in 0..4 {
				ham_distance += calc_ham_dist(input_chunks[y].to_vec(), input_chunks[z].to_vec());

			}
		}

		ham_distance = (ham_distance / 4) / x as i32;

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

		// println!("ham_distance: {:?}, key: {:?}", ham_distance, x);
	}

	// println!("{:?}", ret_key_sizes);

	return ret_key_sizes;
}

fn decrypt_data(key_sizes: &Vec<i32>, input: &str) -> String {
	let input_bytes_arr = base64::decode(&input).unwrap();

	for key in key_sizes.iter() {
		let input_chunks: Vec<&[u8]> = input_bytes_arr.chunks(*key as usize).collect();
		// println!("numchunks: {:?},\n{:?}", input_chunks.len(), input_chunks);

		//need to form X blocks comprised of byte N from each chunk
		for x in 0..*key {
			let mut new_chunk: Vec<u8> = b"".to_vec();

			for chunk in &input_chunks {
				//need to have if condition to make sure the index doesn't go out of index of the chunk
				if x < chunk.len() as i32 {
					new_chunk.push(chunk[x as usize]);
				}
			}

			// println!("new chunk (len {:?}): {:?}", new_chunk.len(), new_chunk);
		}
	}

	return "".to_string();
}

fn open_file(file_name: &str) -> io::Result<String>{
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let mut cur_line: String = "".to_string();

	for line in reader.lines() {
		cur_line = format!("{}{}", cur_line, line.unwrap());
	}

	// println!("{:?}", cur_line);

	return Ok(cur_line);
}

//need to change xor_data function from challenge 3 to return the xor'd character which will correspond to a single character of the key
//need to run that function x times for length of key, building the key one character at a time
//change the scoring system to count the raw frequency of character as the best score --> can ignore frequency * english frequency

fn main() {
	// let s1 = "this is a test".as_bytes().to_vec();
	// let s2 = "wokka wokka!!!".as_bytes().to_vec();
	// let s3 = "this is a test".as_bytes().to_vec();
	// let s4 = "wokka wokka!!!".as_bytes().to_vec();
	//60 base64 chars per line => 60 chars / 4 chars/3 bytes = 45 bytes
	// let test_str = "HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS"

	let file_str = open_file("full_input.txt").unwrap();

	let key_sizes = find_key_size(&file_str);
	decrypt_data(&key_sizes, &file_str);


}
