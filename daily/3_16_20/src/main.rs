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

	//put in separate initialization function
	for i in 0..input1.len() {
		for j in 0..input2.len() {
			if j == 0 {
				memoize[i + 1][j] = input1[i]
			}

			if i == 0 {
				memoize[i][j + 1] = input2[j]; 
			}
		}
	}

	print_results(memoize);

	return ret_subsequence;
}

fn main() {
	let vec1 = vec![1, 2, 3, 4, 5];
	let vec2 = vec![6, 7, 8];
	find_subsequence(vec1, vec2);
}
