fn sort(mut data: &mut Vec<i64>, left: i32, right: i32) {
	if left < right {
		let middle: i32 = (left + right) / 2;

		sort(&mut data, left, middle);
		sort(&mut data, middle + 1, right);
		merge(&mut data, left, middle, right);
	}

}

fn merge(mut data: &mut Vec<i64>, left: i32, middle: i32, right: i32) {
	let mut left_arr: Vec<i64> = Vec::new();
	let mut right_arr: Vec<i64> = Vec::new();
	let mut cur_idx: i64 = left as i64;



	for i in left_arr.iter() {
		cur_idx += 1;
	} 

	for i in right_arr.iter() {
		cur_idx += 1;
	}
}

fn havel_hakimi_algo(input: &mut Vec<i64>) -> bool {
	let remove_zero_var = 0;

	//remove 0s from the array
	input.retain(|&x| x != remove_zero_var);

	//sort the vector
	input.sort();

	for j in input.iter() {
		print!("output: {:?}\n", j);
	}

	// input.sort()

	return true;
}

fn main() {
	let mut input: Vec<i64> = Vec::new();
	input.push(0);
	input.push(15);
	input.push(2);
	input.push(0);
	input.push(5);
	input.push(15);


	havel_hakimi_algo(&mut input);
}
