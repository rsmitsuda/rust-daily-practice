//find first duplicate in an array

use std::collections::HashMap;

fn find_duplicates(arr: &[i32]) -> i32 {
	// print!("{:?}", arr[0]);
	let mut seen: HashMap<i32, bool> = HashMap::new();

	for num in arr.iter() {
		if !seen.contains_key(num) {
			seen.insert(*num, true);
		} else {
			return *num;
		}
	}

	return 0;
}

fn main() {
	let arr: [i32; 5] = [1, 3, 2, 3, 2];

	print!("first duplicate: {:?}\n", find_duplicates(&arr));
}

