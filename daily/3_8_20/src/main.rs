//problem: https://www.reddit.com/r/dailyprogrammer/comments/cdieag/20190715_challenge_379_easy_progressive_taxation/

fn round_taxed_money(cur: f64) -> f64 {
	return (cur * 100.0).round() / 100.0;
}

fn taxation(total: f64) -> f64 {
	let mut test: f64 = total.clone();
	let mut cur_total: f64 = 0 as f64;
	let mut taxable_money: f64 = 0 as f64;

	if total >= 100000 as f64 {
		taxable_money = total - 100000 as f64; 
		cur_total += round_taxed_money(taxable_money * 0.4);
		test -= taxable_money;
		// print!("cur_Total: {:?}, taxable_Money: {:?}\n", cur_total, test);

	}

	if total > 30000.0 {
		taxable_money = test - 30000.0;
		cur_total += round_taxed_money(taxable_money * 0.25);
		test -= taxable_money;
		// print!("cur_Total: {:?}, taxable_Money: {:?}\n", cur_total, test);

	}

	if total > 10000.0 {
		taxable_money = test - 10000.0;
		cur_total += round_taxed_money(taxable_money * 0.1);
		test -= taxable_money;
		// print!("cur_Total: {:?}, taxable_Money: {:?}\n", cur_total, test);
	}

	if total > 0 as f64 {
		cur_total += round_taxed_money(taxable_money * 0 as f64);
		// print!("cur_Total: {:?}, taxable_Money: {:?}\n", cur_total, test);
	}

	return total;
}

fn main() {
	taxation(101000 as f64);
	taxation(1234567 as f64);
}