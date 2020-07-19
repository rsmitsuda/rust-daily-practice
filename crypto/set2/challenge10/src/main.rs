use std::str;
use openssl::symm::{decrypt, Cipher, encrypt, Crypter, Mode};
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


fn xor_data(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
	let mut ret_str: String = "".to_string();
	let mut temp_vec: Vec<u8> = Vec::new();

	for (x, y) in vec1.iter().zip(vec2.iter()) {
		temp_vec.push(x ^ y);
	}

	return temp_vec;
}

fn cbc_decrypt(cipher: openssl::symm::Cipher, iv: Option<&[u8]>, data: &[u8], key: &[u8]) {
	//need to break this into blocks of 128 bits (16 byte chunks)
	let data_chunks: Vec<&[u8]> = data.chunks(16).collect();
	let mut change_iv = iv.unwrap().clone().to_vec();
	let mut aes = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
	let mut ret_str: String = "".to_string();

	for chunk in data_chunks {
		println!("chunk1: {:?}", chunk);
		println!("cur iv {:?}", change_iv);
		let mut ret_thing = vec![0 as u8; chunk.len() + 16];
		aes.update(&chunk, &mut ret_thing);
		let test_me = xor_data(&ret_thing, &change_iv);
		change_iv = chunk.to_vec();
		println!("change iv: {:?}", change_iv);
		println!("{:?}", str::from_utf8(&test_me));

		ret_str = format!("{}{}", ret_str, str::from_utf8(&test_me).unwrap());
		// ret_thing.clear();
		aes = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
	}

	println!("final string: {:?}", ret_str);
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

	cbc_decrypt(cipher, Some(iv), data, key);


	// cbc_decrypt(cipher, Some(iv), &ciphertext, key);
}
