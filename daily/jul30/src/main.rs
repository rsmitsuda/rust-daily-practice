fn problem(input: &str) -> String {
	let split_str = input.split(" ").collect::<Vec<&str>>().to_vec();
	let mut stringbuilder: String = "".to_string();
	
	for x in split_str {
		let temp = stringbuilder.clone();
		stringbuilder = format!("{}{} ", temp, (x.chars().rev().collect::<String>()));
	}

	stringbuilder[..stringbuilder.len()-1].to_string()
}

fn main() {
	println!("{:?}", problem(&"this is a test"));
}
