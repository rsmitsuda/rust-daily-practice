use std::str;
use openssl::symm::{decrypt, Cipher};
use base64;
use std::fs::File;
use std::fs;
use std::io::{self, prelude::*, BufReader};

//block size is 128 bits (16 bytes)
const BLOCKSIZE: u8 = 16;

fn open_file(file_name: &str) -> io::Result<String>{
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let mut cur_line: String = "".to_string();

	for line in reader.lines() {
		cur_line = format!("{}{}", cur_line, line.unwrap());
	}

	return Ok(cur_line);
}

fn cbc_decrypt(cipher: openssl::symm::Cipher, iv: Option<&[u8]>, data: &[u8], key: &[u8]) {
	//need to break this into blocks of 128 bits
	let ciphertext = decrypt(
		cipher,
		key,
		iv,
		data
		).unwrap();
}

fn cbc_encrypt(cipher: openssl::symm::Cipher, iv: &[u8]) {

}

fn insert_padding(input: &str, blocksize: i8) -> String {
	let mut num_pad_bytes: i8 = blocksize - (input.len() as i8 % blocksize);
	let pad_byte = num_pad_bytes.to_ne_bytes();
	let mut ret_str: Vec<u8> = input.clone()
		.to_string()
		.as_bytes()
		.to_vec();

	while num_pad_bytes > 0 {
		ret_str.push(pad_byte[0]);
		num_pad_bytes -= 1;
	}

	return str::from_utf8(&ret_str).unwrap().to_string();
}

fn main() {
	let cipher = Cipher::aes_128_ecb();
	let key: &[u8] = b"YELLOW SUBMARINE";
	let iv: &[u8] = &vec![0x00; BLOCKSIZE as usize];
	
	let data:&[u8] = &base64::decode(open_file("input.txt").unwrap()).unwrap();
	println!("{:?}", data);
	cbc_decrypt(cipher, Some(iv), data, key);

}
