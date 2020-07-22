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

fn create_secret(blocksize: i32, oracle: &mut EcbOracle) {
	//get len of ciphertext with no additional input
	let mut secret_builder: Vec<u8> = Vec::new();

	//this is the length of cyphertext created with no added data
	let length_to_be_found = oracle.oracle_encrypt(&secret_builder).len();

	//idx to keep track of which byte we need to decode
	let mut secret_builder_idx = blocksize - 1 - (secret_builder.len() as i32 % blocksize);

	//vector that will be used as input data into cypher to decode the unknown string
	let mut byte_by_byte_vec = vec![b'A'; secret_builder_idx as usize];

	//vector of byte blocks 
	let mut final_secret: Vec<Vec<u8>> = Vec::new();





	for i in 0..length_to_be_found {
		//cycle through all possible bytes 
		println!("secret_builder_len {:?}", secret_builder_idx);
		let mut input_vec = vec![b'A'; secret_builder_idx as usize];
		let ciphertext = oracle.oracle_encrypt(&input_vec);
		//need to add one additional space to the vector (for the new_ciphertext) since this is the byte that we will use to decode the ciphertext
		// input_vec.push(b'A');

		for j in 0..255 {
			// let mut test_vec = vec![b'A'; blocksize as usize];
			// test_vec[15] = ((j as u8).to_ne_bytes())[0];
			//this is the initial cyphertext -- looking for byte16 (byte at index 15)
			input_vec.extend(&secret_builder);
			input_vec.push(((j as u8).to_ne_bytes())[0]);

			if i == 0 {
				println!("{:?}", input_vec);

			}
			let new_ciphertext = oracle.oracle_encrypt(&input_vec);
			// println!("{:?}", input_vec);

			if i == 0 {
				// println!("{:?}", byte_by_byte_vec);
				// println!("first: {:?} second: {:?}", blocksize as usize * final_secret.len() + secret_builder_idx as usize, secret_builder_idx);


			}
			//oracle.oracle_encrypt(byte_by_byte_vec)



			// if ciphertext[blocksize as usize * final_secret.len() + secret_builder_idx as usize] == new_ciphertext[secret_builder_idx as usize] {
			if ciphertext[.. (blocksize as usize * final_secret.len() + secret_builder_idx as usize)] == new_ciphertext[.. secret_builder_idx as usize] {

				//prepend byte j to the secretbuilder
				println!("WE FOUND A MATCH ({:?}): {:?}", i, ((j as u8).to_ne_bytes())[0]);
				secret_builder.push(((j as u8).to_ne_bytes())[0]);
				 println!("first: {:?} second: {:?}", blocksize as usize * final_secret.len() + secret_builder_idx as usize, secret_builder_idx);

				break;
			}

			input_vec = vec![b'A'; secret_builder_idx as usize];

		}
		// byte_by_byte_vec = vec![b'A'; blocksize as usize];
		secret_builder_idx = blocksize - 1 - (secret_builder.len() as i32 % blocksize);

		if secret_builder.len() as i32 == blocksize {
			final_secret.push(secret_builder.clone());
			secret_builder.clear();
			println!("FOUND ONE FULL BLOCK");
		}
	}

	// println!("{:?}", ciphertext);
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
    create_secret(blocksize, &mut oracle);

}
