extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn yahtzee_upper(data: &Vec<i32>) -> i32{
	let mut totals: HashMap<i32, i32> = HashMap::new();
	let (mut temp, mut max) : (i32, i32) = (0, 0);

	for key in data {
		if !totals.contains_key(&key) {
			totals.insert(*key, 1);
		} else {
			match totals.get_mut(&key) {
				Some(v) => *v += 1,
				None => (),
			}
		}
	}

	for (key, value) in totals.iter() {
		temp = key * value;
		if temp > max {
			max = temp;
		}
		print!("key: {:?}, value: {:?}\n", key, value);

	}
	return max;
}

fn main() {
	let mut vec: Vec<i32> = Vec::new();

	for _i in 0..5 {
		vec.push(rand::thread_rng().gen_range(1, 7));
	}

	print!("{:?}\n", yahtzee_upper(&vec));
}
