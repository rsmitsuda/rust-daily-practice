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

		// println!("combined_data: {:?}", combined_data);
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

fn create_secret(blocksize: i32, oracle: &mut EcbOracle) -> Vec<u8> {
	//get len of ciphertext with no additional input
	let mut secret_builder: Vec<u8> = Vec::new();

	//this is the length of cyphertext created with no added data
	let length_to_be_found = oracle.oracle_encrypt(&secret_builder).len();

	//idx to keep track of which byte we need to decode
	let mut secret_builder_idx = blocksize - 1 - (secret_builder.len() as i32 % blocksize);

	let mut found_char = false;

	for i in 0..length_to_be_found {
		let mut input_vec = vec![b'A'; secret_builder_idx as usize];
		let ciphertext = oracle.oracle_encrypt(&input_vec);

		for j in 0..256 {
			input_vec.extend(&secret_builder);
			input_vec.push(((j as u8).to_ne_bytes())[0]);

			let new_ciphertext = oracle.oracle_encrypt(&input_vec);

			//check if whole blocks are equal
			if ciphertext[..(secret_builder.len() + secret_builder_idx as usize + 1)] == new_ciphertext[..(secret_builder.len() + secret_builder_idx as usize + 1)] {
				secret_builder.push(((j as u8).to_ne_bytes())[0]);
				found_char = true;
				break;
			} 

			input_vec = vec![b'A'; secret_builder_idx as usize];
		}

		//inputting 1 as the last char because of padding.. it puts 1 because the last block will be real block sans one character, then add padding (x01)
		//fails afterwards because padding will change, so they will never be equal again because previous char will have changed in new ciphertext
		//example AAAA AAAA AAAA AAA(x01) => AAAA AAAA AAAA AA(x02)(x02) (byte at idx 14 was decoded as 1, but changes to 2 in the new ciphertext because of padding)
		//we can use found_char bool to know when we really finished and remove the last byte because we know it is padding if we can't decode another character
		if !found_char {
			secret_builder.remove(secret_builder.len() - 1);
			break;
		} else {
			found_char = false;
		}

		secret_builder_idx = blocksize - 1 - (secret_builder.len() as i32 % blocksize);

	}

	return secret_builder;
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
    // println!("blocksize: {:?}", blocksize);

    //detect cipher mode
    let repeat_data = vec![b'0'; 10000];
    let temp_ciphertext = oracle.oracle_encrypt(&repeat_data);
    find_aes_mode(&temp_ciphertext);

    //start to build the secret unknown
    let built_secret = create_secret(blocksize, &mut oracle);
    println!("{:?}", built_secret);

    assert!(built_secret == temp);
}
