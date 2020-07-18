use std::str;

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
	let input = "YELLOW SUBMARINE";

	println!("{:?}", insert_padding(&input, 20));
}
