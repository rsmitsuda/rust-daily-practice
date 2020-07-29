fn is_palindrome(input: &str) -> bool {
	let result = true;
	let mut end = input.len() - 1;
	let input_arr: Vec<u8> = input.as_bytes().to_vec();

	for i in 0..input.len() {
		// println!("{:?}", input_arr[i]);
		if i >= end {
			break;
		} else {
			if input_arr[i] != input_arr[end] {
				return false;
			}

			end -= 1;
		}
	}

	return true;
}


fn main() {
    println!("Hello");

    println!("{:?}", is_palindrome(&"hello"));
        println!("{:?}", is_palindrome(&"ll"));
            println!("{:?}", is_palindrome(&"helleh"));
}
