use std::default::{Default};
use rand;
use openssl::symm::{decrypt, Cipher, encrypt, Crypter, Mode};

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

fn kv_parse(cookie: &str) -> String {
	let temp: Vec<&str> = cookie.split('&').collect();
	for x in temp.iter() {
		println!("{:?}", x);
	}
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

fn main() {
	// let mut ret_str = "{\n\temail: ";
	// println!("{}", ret_str);
	kv_parse("foo=bar&baz=qux&zap=zazzle");

    println!("{}", profile_for(&"test"));
}
