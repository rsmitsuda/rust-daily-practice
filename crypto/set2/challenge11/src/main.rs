use rand;
use rand::Rng;
use openssl::symm::{decrypt, Cipher, encrypt, Crypter, Mode};
use std::collections::HashSet;


fn generate_data() -> Vec<u8> {
	let mut data = Vec::new();
	//plaintext will be same byte so that we can detect ECB
	let mut plaintext: Vec<u8> = vec![rand::random::<u8>(); 16 * 10000 - 5];

	//generate 5 random bytes and prepend to plaintext
	for i in 0..5 {
	    let x = rand::random::<u8>();
   		data.push(x);
	}

	data.append(&mut plaintext);

	return data;
}

fn random_encrypt(data: &[u8]) -> Vec<u8> {
	let mut key = Vec::new();
	let mut ciphertext = Vec::new();
	let mut iv = Vec::new();
	let encrypt_mode = rand::thread_rng().gen_range(0, 2);

	//generate random key
	for i in 0..16 {
		key.push(rand::random::<u8>());
	}

	if encrypt_mode == 0 {
		//genearte random iv
		for i in 0..16 {
			iv.push(rand::random::<u8>());
		}
		ciphertext = encrypt(
			Cipher::aes_128_cbc(),
			&key,
			Some(&iv),
			data
			).unwrap();
		println!("{:?}", "CBC");
	} else {
		ciphertext = encrypt(
			Cipher::aes_128_ecb(),
			&key,
			None,
			data
			).unwrap();
		println!("{:?}", "ECB");
	}

	// println!("{:?}", ciphertext);
	return ciphertext;
}

fn find_aes(input_vec: &Vec<u8>) {
	let chunks: Vec<&[u8]> = input_vec.chunks(16).collect();
	let chunk_set: HashSet<&[u8]> = 
		chunks.to_vec()
		.into_iter()
		.collect();

	println!("{:?}", chunk_set.len());
	if chunk_set.len() > 3 {
		println!("{:?}", "CBC");
	} else {
		println!("{:?}", "ECB");
	}
}

fn main() {
	let data = generate_data();
	let ciphertext = random_encrypt(&data);
	find_aes(&ciphertext);
}
