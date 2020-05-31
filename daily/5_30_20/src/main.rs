//https://leetcode.com/problems/palindrome-number/

struct Solution {

}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
    	let mut num_builder = 0;
		let mut temp_num = x.clone();

		if x < 0 {
			return false;
		}

		if x % 10 == 0 {
			return true;
		} else {
			loop {
				if temp_num % 10 == 0 {
					break;
				}

		    	//build string logic
				let ones_place = temp_num % 10;
		    	num_builder = num_builder * 10 + ones_place; 
		    	// println!("num_builder: {:?}", num_builder);
				temp_num = temp_num / 10;
	    		// println!("temp_num: {:?}", temp_num);
			}
	    	println!("num_builder: {:?}", num_builder);
		}
    	

    	if num_builder == x {
    		return true;
    	}

        return false;
    }
}

fn main() {
	println!("{}", Solution::is_palindrome(123454321));
}
