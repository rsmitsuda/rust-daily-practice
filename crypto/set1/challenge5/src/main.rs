use hex;

fn encrypt_string(key: Vec<u8>, input_str: Vec<u8>) -> String {
	let mod_val = key.len();
	let mut ret_vec = Vec::new();

	for x in 0..input_str.len() {
		ret_vec.push(key[x % mod_val] ^ input_str[x]);
	}

	return hex::encode(&ret_vec);
}


fn main() {
	let key = vec![b'I', b'C', b'E'];
	let input_str = b"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal".to_vec();

    println!("{:?}", encrypt_string(key, input_str));
}
