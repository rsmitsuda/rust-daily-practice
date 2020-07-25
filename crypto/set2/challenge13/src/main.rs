use std::default::{Default};
use rand;
use openssl::symm::{decrypt, Cipher, encrypt, Crypter, Mode};
use std::str;

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

	fn oracle_decrypt(&self, ciphertext: &[u8]) -> String {
		let mut aes = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &self.key, None).unwrap();
		let data_chunks: Vec<&[u8]> = ciphertext.chunks(16).collect();
		let mut ret_str: String = "".to_string();

		for chunk in data_chunks {
			let mut ret_thing = vec![0 as u8; chunk.len() + 16];
			aes.update(&chunk, &mut ret_thing);
			ret_str = format!("{}{}", ret_str, str::from_utf8(&ret_thing).unwrap());
			// ret_thing.clear();
			aes = Crypter::new(Cipher::aes_128_ecb(), Mode::Decrypt, &self.key, None).unwrap();
		}

		ret_str
	}
}

fn kv_parse(cookie: &str) -> String {
	let temp: Vec<&str> = cookie.split('&').collect();
	let mut ret_vec: Vec<&str> = Vec::new();
	// let thingy: Vec<&str> = temp.into_iter().map(|x| x.split('=')).collect();
	for x in temp.iter() {
		// println!("{:?}", x);
		let temp_vec: Vec<&str> = x.split('=').collect();
		ret_vec.extend(&temp_vec);

	}

	for i in ret_vec {
		println!("{:?}", i);
	}
	// let ret_str = profile_for(&ret_vec, &"test");
	let ret_str = profile_for(&"test");
	return ret_str;
}

//replace values with values from kv_parse
fn profile_for(email: &str) -> String {
	let prefix = "{\n\temail: \'";
	let suffix = "\'\n\tuid: 10\n\trole: \'admin\'\n}";
	return format!("{}{}{}", prefix, email, suffix);
	// return ret_str.to_string();
}

//other profile function
/*fn profile_for(fields: &Vec<&str>) -> String {
	let prefix = format!("{{\n\t{}: \'", fields[0]);
	let suffix = format!("\'\n\t{}: {}\n\t{}: \'{}\'\n}}", fields[2], fields[3], fields[4], fields[5]);
	return format!("{}{}{}", prefix, fields[1], suffix);
	// return ret_str.to_string();
}*/

// -------------------------------------------------------------------------
// email=food@bar.co m&uid=10&role=us er
// 01234567890123450 0123456789012345 01
//                   |
// want to turn into |
//                   v 
// need to have role= be the end of the block and admin start at beginning of block
// will create email long enough to cut, copy, paste blocks to create the new CT
// 
// email=XXXXXXXXXX foo@bar.comXXXXX adminXXXXXXXXXXX XXXXXXXXXXXXXXXX XXX&uid=10&role= user
// 0123456789012345 0123456789012345 0123456789012345 0123456789012345 0123456789012345 01234
//   B1                    B2              B3              B4               B5
//  B1 + B2 + B5 + B3
//--------------------------------------------------------------------------

fn main() {
	// let mut ret_str = "{\n\temail: ";
	// println!("{}", ret_str);
	kv_parse("foo=bar&baz=qux&zap=zazzle");

	let eng_string = b"email=XXXXXXXXXXfoo@bar.comXXXXXadminXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXuid=10&role=user";
    let mut oracle = EcbOracle::default();
    oracle.generate_key();

    let ciphertext= oracle.oracle_encrypt(eng_string);
    println!("{:?}", ciphertext);

    let first = &ciphertext[0..32];
    println!("{:?}, {}", first, first.len());

    println!("decrypt: {}", oracle.oracle_decrypt(&first));

    // println!("{}", profile_for(&"test"));
}
