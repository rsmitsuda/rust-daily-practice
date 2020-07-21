use base64;
use std::default::{Default};
use rand;
use openssl::symm::{decrypt, Cipher, encrypt, Crypter, Mode};
use std::collections::HashSet;

#[derive(Default)]
struct EcbOracle {
	unknown: Vec<u8>,
	key: Vec<u8>,
}

impl EcbOracle {
	fn unknown_mut(&mut self) -> &mut Vec<u8> {
		&mut self.unknown
	}

	fn key_mut(&mut self) -> &mut Vec<u8> {
		&mut self.key
	}

	fn generate_key(&mut self) {
		let mut key = Vec::new();

		for i in 0..16 {
			let x = rand::random::<u8>();
			key.push(x);
		}

		self.key = key;
	}

	fn oracle_encrypt(&mut self, data: &[u8]) -> Vec<u8> {
		let mut combined_data = data.clone().to_vec();

		for x in self.unknown.iter() {
			combined_data.push(*x);
		}

		let ciphertext = encrypt(
			Cipher::aes_128_ecb(),
			&self.key,
			None,
			&combined_data
			).unwrap();

		ciphertext
	}
}

fn find_block_size(oracle: &mut EcbOracle) -> (i32, Vec<u8>) {
	let mut test_byte = vec![b'A'];
	let mut ciphertext = oracle.oracle_encrypt(&test_byte);
	let init_len = ciphertext.len();
	let mut cur_len = ciphertext.len();

	while init_len == cur_len {
		test_byte.push(b'A');
		ciphertext = oracle.oracle_encrypt(&test_byte);
		cur_len = ciphertext.len();
	}

	// println!("cur: {:?}, init: {:?}", cur_len, init_len);
	return (cur_len as i32 - init_len as i32, ciphertext);

}

fn find_aes_mode(input_vec: &Vec<u8>) {
	let chunks: Vec<&[u8]> = input_vec.chunks(16).collect();
	let chunk_set: HashSet<&[u8]> = 
		chunks.to_vec()
		.into_iter()
		.collect();



	println!("{:?}", chunk_set.len());
	if chunk_set.len() != chunks.len() {
		println!("{:?}", "ECB");
	} else {
		println!("{:?}", "CBC");
	}
}

fn main() {
    let unknown = base64::decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap();
    let temp = unknown.clone();
    //create oracle
    let mut oracle = EcbOracle::default();

    //assign unknown value to oracle
    *oracle.unknown_mut() = unknown;

    //generate unknown key for oracle
    oracle.generate_key();

    //print blocksize
    let (blocksize, ciphertext): (i32, Vec<u8>) = find_block_size(&mut oracle);
    println!("blocksize: {:?}", blocksize);

    //detect cipher mode
    let repeat_data = vec![b'0'; 10000];
    let temp_ciphertext = oracle.oracle_encrypt(&repeat_data);
    find_aes_mode(&temp_ciphertext);
}
