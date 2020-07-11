use base64;

fn convert_to_64(hex_string: &str) -> String {
	let mut byte_vec = Vec::new();

	for i in (0..hex_string.len()).step_by(2) {
		let decimal_byte = u8::from_str_radix(&hex_string[i..i+2], 16);
		println!("{:?}", decimal_byte);

		match decimal_byte {
			Ok(val) => byte_vec.push(val),
			Err(_) => println!("{:?}", "there was a problem with string size (not divisible)"),
		}
	}

	return base64::encode(&byte_vec);

	// println!("{:?}", hex_string_as_bytes);
}


fn main() {
	let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let base64_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert!(convert_to_64(&hex_string) == base64_string);
}
