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
	let (mut left_idx, mut right_idx): (usize, usize);
	let mut cur: usize = left.clone() as usize;

	//need to put left and right side of data into temp arrays (left and right)
	for i in (left as usize)..=(right as usize) {
		// print!("i: {:?} num: {:?} \n", i, data[i]);

		if i < (middle + 1) as usize {
			left_arr.push(data[i]);
		} else {
			right_arr.push(data[i]);
		}
	}


	//print out the contents of the array [test]
/*	{
		print!("left:");

		for ele in left_arr.iter() {
			print!("{:?} ", ele);
		}

		print!("\n");
		print!("right:");

		for ele in right_arr.iter() {
			print!("{:?} ", ele);
		}
		print!("\n");

	}*/

	left_idx = 0;
	right_idx = 0; 

	//need to sort the left and right temp arrays with loop
	//while left arr is not empty or right arr is not empty then continue else break
	//go to final loops to move rest of data
	while (left_idx < left_arr.len()) && (right_idx < right_arr.len()) {
		if left_arr[left_idx] > right_arr[right_idx] {
			data[cur] = left_arr[left_idx];
			left_idx += 1;
		} else {
			data[cur] = right_arr[right_idx];
			right_idx += 1;
		}
		cur += 1;

	} 

	//pushing rest of unfinished data into the main data vector
	while left_idx < left_arr.len() {
		data[cur] = left_arr[left_idx];
		left_idx += 1;
		cur += 1;
	} 

	//change this to right_idx
	while right_idx < right_arr.len() {
		data[cur] = right_arr[right_idx];
		right_idx += 1;
		cur += 1;
	}

	// 	{
	// 	for ele in data.iter() {
	// 		print!("{:?} ", ele);
	// 	}
	// }
}

fn havel_hakimi_algo(mut input: &mut Vec<i64>) -> bool {
	let remove_zero_var = 0;

	//remove 0s from the array
	input.retain(|&x| x != remove_zero_var);

	sort(&mut input, 0, 4);

	//sort the vector
	// input.sort();

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
	input.push(11);
	// input.push()


	havel_hakimi_algo(&mut input);
}
