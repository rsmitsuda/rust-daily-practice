use hex;
use std::fs::File;
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

//need to find set of values with least amount of unique values => ecb will have repeating blocks of ciphertext when same plaintext is encrypted
fn find_aes(input_vec: Vec<String>) -> String {
	let mut unique_vals = 10000;
	let mut ret_line = "".to_string();

	for line in input_vec {
		let bytes = hex::decode(line.clone()).unwrap();
		// println!("{:?}", bytes.len());

		let chunks: Vec<&[u8]> = bytes.chunks(16).collect();
		//see which chunk has the most repeat values of 16 bytes
		let chunk_set: HashSet<&[u8]> = 
			chunks.to_vec()
			.into_iter()
			.collect();

		if chunk_set.len() < unique_vals {
			unique_vals = chunk_set.len();
			ret_line = line;
		}
	}

	return ret_line;
}


fn open_file(input: &str) -> io::Result<Vec<String>> {
	let file = File::open(input)?;
	let reader = BufReader::new(file);
	let mut ret_vec: Vec<String> = Vec::new();

	for line in reader.lines() {
		ret_vec.push(line.unwrap());

	}

	return Ok(ret_vec);
}

fn main() {
	println!("{:?}", find_aes(open_file(&"input.txt").unwrap()));

}
