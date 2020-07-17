use hex;
use std::fs::File;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn find_aes(input_vec: Vec<String>) {
	for line in input_vec {
		let bytes = hex::decode(line.clone()).unwrap();
		println!("{:?}", bytes.len());

		let chunks: Vec<&[u8]> = bytes.chunks(16).collect();
		println!("chunks: {:?}", chunks);
		//see which chunk has the most repeat values of 16 bytes

	}

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
	find_aes(open_file(&"input.txt").unwrap());

}
