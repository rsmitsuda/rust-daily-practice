extern crate vectors;

use std::vec::Vec;

fn print_results(results: &Vec<Vec<i64>>) {
	for i in 0..results.len() {
		for j in 0..results[0].len() {
			print!("{:?}", results[i][j]);
		}
		print!("\n");
	}
}

fn backtrack(dp: &Vec<Vec<i64>>, input1: &Vec<i64>) -> Vec<i64> {
	let (mut rows, mut cols) = (dp.len() - 1, dp[0].len() - 1);
	let (mut diag, mut hrzn, mut vert, mut cur): (i64, i64, i64, i64);
	let mut rev: Vec<i64> = Vec::new();
	let mut ret: Vec<i64> = Vec::new();


	while rows != 0 && cols != 0 {
		cur = dp[rows][cols];
		diag = dp[rows - 1][cols - 1];
		hrzn = dp[rows - 1][cols];
		vert = dp[rows][cols - 1];

		if diag == hrzn && diag == vert {
			rev.push(input1[rows - 1]);
			rows -= 1;
			cols -= 1;
		} else {
			if hrzn > vert {
				rows -= 1;
			} else {
				cols -= 1;
			}
		}
	}

	let ctr = rev.len() - 1;

	for i in rev.iter().rev() {
		ret.push(*i);
	}

	return ret;

}

fn find_subsequence(input1: Vec<i64>, input2: Vec<i64>) -> Vec<i64> {
	let mut ret_subsequence: Vec<i64> = Vec::new();
	let mut memoize = vec![vec![0; input2.len() + 1]; input1.len() + 1];
	let (mut cur_i, mut cur_j): (i64, i64);

	for i in 1..memoize.len() {
		cur_i = input1[i - 1];

		for j in 1..memoize[0].len() {
			cur_j = input2[j - 1];

			// print!("({:?}, {:?}) \n", cur_i, cur_j);

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

	ret_subsequence = backtrack(&memoize, &input1);
	print_results(&memoize);

	return ret_subsequence;
}

fn main() {
	let vec1 = vec![1, 2, 20, 23, 3, 24];
	let vec2 = vec![1, 20, 23, 24];
	for i in (find_subsequence(vec1, vec2)).iter() {
		print!("{:?}, ", i);
	}
}
