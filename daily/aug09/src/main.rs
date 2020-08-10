fn find_ones(lower: i32, upper: i32) -> i32 {
	let mut ret_val = 0;
	for x in lower..upper + 1 {
		// println!("{:?}", x.to_string());
		let char_arr: Vec<char> = x.to_string()
			.chars()
			.collect();

		for y in char_arr.iter() {
			if *y == '1' {
				ret_val += 1;
			}
		}
	}

	ret_val
}

fn main() {
	println!("{:?}", find_ones(1, 100));
}
