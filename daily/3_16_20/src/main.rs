extern crate vectors;

use std::vec::Vec;

fn print_results(results: Vec<Vec<i64>>) {
	for i in 0..results.len() {
		for j in 0..results[0].len() {
			print!("{:?}", results[i][j]);
		}
		print!("\n");
	}
}

fn find_subsequence(input1: Vec<i64>, input2: Vec<i64>) -> Vec<i64> {
	let mut ret_subsequence: Vec<i64> = Vec::new();
	let mut memoize = vec![vec![0; input2.len() + 1]; input1.len() + 1];
	let (mut cur_i, mut cur_j): (i64, i64);

	for i in 1..memoize.len() {
		// cur_i = memoize[i][0];
		cur_i = input1[i - 1];
		for j in 1..memoize[0].len() {
			// cur_j = memoize[0][j];
			cur_j = input2[j - 1];

			print!("({:?}, {:?}) \n", cur_i, cur_j);

			if cur_i == cur_j {
					if memoize[i - 1][j] > memoize[i][j - 1] {
						if memoize[i - 1][j - 1] > memoize[i - 1][j] {
							memoize[i][j] = memoize[i - 1][j - 1] + 1;
						} else {
							memoize[i][j] = memoize[i - 1][j] + 1;
						}
					} else {
						if memoize[i - 1][j - 1] > memoize[i][j - 1] {
							memoize[i][j] = memoize[i - 1][j - 1] + 1;
						} else {
							memoize[i][j] = memoize[i][j - 1] + 1;
						}
					}
			} else {
					if memoize[i - 1][j] > memoize[i][j - 1] {
						if memoize[i - 1][j - 1] > memoize[i - 1][j] {
							memoize[i][j] = memoize[i - 1][j - 1];
						} else {
							memoize[i][j] = memoize[i - 1][j];
						}
					} else {
						if memoize[i - 1][j - 1] > memoize[i][j - 1] {
							memoize[i][j] = memoize[i - 1][j - 1];
						} else {
							memoize[i][j] = memoize[i][j - 1];
						}
					}
			}
		}
	}

	print_results(memoize);

	return ret_subsequence;
}

fn main() {
	let vec1 = vec![1, 2, 20, 23, 3, 24];
	let vec2 = vec![1, 20, 23, 24];
	find_subsequence(vec1, vec2);
}
