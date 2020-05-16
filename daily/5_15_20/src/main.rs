fn compare(s1: &str, s2: &str) -> bool {
	/*if (s1.len() != s2.len()) {
		return false;
	}*/

	for i in 0..s1.len() {
		let (first, second) = s1.split_at(i);
		let temp_str = second.to_owned() + first;
		if (temp_str == s2) {
			return true;
		}
		// print!("first: {:?}.. second: {:?}\n", first, second);

	}

	return false;
}

fn main() {
	let s1 = "hello";
	// let s2 = "goodbye";
	let s2 = "ohell";

	print!("{:?}", compare(&s1, &s2));
}
