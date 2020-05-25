fn sum_num() -> {
    //divide by 10 and if it is 0, then it is a single digit number and return the counter
    
}

fn persistence(mut num: i32) -> i32 {
    let mut counter = 0;
    
    sum_num(num);
    while let (mut cur) = num % 10 {
        print!("num first: {:?}\n", num);

        if cur > 0 {
            print!("cur: {:?}\n", cur);
            if (sum(num) == 1) {
                return counter;
            };
            num = num / 10;
        } else {
            break;
        }
    }

    return 0;
}

fn main() {
    print!("test {:?}\n", 123999 / 10);

    persistence(123999);
}
