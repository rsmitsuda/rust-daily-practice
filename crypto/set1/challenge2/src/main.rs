use hex;

fn convert_to_u8(hex_string: &str) -> Vec<u8> {
	let mut byte_vec = Vec::new();

	for i in (0..hex_string.len()).step_by(2) {
		let decimal_byte = u8::from_str_radix(&hex_string[i..i+2], 16);
		println!("{:?}", decimal_byte);

		match decimal_byte {
			Ok(val) => byte_vec.push(val),
			Err(_) => println!("{:?}", "there was a problem with string size (not divisible)"),
		}
	}

	return byte_vec;
}

fn xor_strings(s1: &str, s2: &str) -> String {
	let s1_from_hex = convert_to_u8(&s1);
	let s2_from_hex = convert_to_u8(&s2);	

	let xor_vals: Vec<u8> = s1_from_hex.iter()
		.zip(s2_from_hex.iter())
		.map(|(&x1, &x2)| x1 ^ x2)
		.collect();

	let ret_str = hex::encode(xor_vals);
	println!("{:?}", ret_str);

	return ret_str.to_string();
}

fn main() {
    let s1 = "1c0111001f010100061a024b53535009181c";
    let s2 = "686974207468652062756c6c277320657965";

    assert!(xor_strings(&s1, &s2) == "746865206b696420646f6e277420706c6179");
}
