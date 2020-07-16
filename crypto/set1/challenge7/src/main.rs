use openssl::symm::{decrypt, Cipher};
use base64;
use std::fs::File;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn open_file(file_name: &str) -> io::Result<String>{
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let mut cur_line: String = "".to_string();

	for line in reader.lines() {
		cur_line = format!("{}{}", cur_line, line.unwrap());
	}

	return Ok(cur_line);
}

fn main() {
    let cipher = Cipher::aes_128_ecb();
	let data: &[u8] = &base64::decode(open_file("input.txt").unwrap()).unwrap();  
	let key: &[u8] = b"YELLOW SUBMARINE";

	let ciphertext = decrypt(
		cipher,
		key,
		None,
		data
		).unwrap();

	let ret_val = String::from_utf8_lossy(&ciphertext);

	println!("{:?}", ret_val);
}

