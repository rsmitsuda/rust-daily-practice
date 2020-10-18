#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_fit(&self, rec2: &Rectangle) -> bool {
		self.width > rec2.width && self.height > rec2. height
	}

	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}
}


fn main() {
    let width1 = 30;
    let height1 = 50;

    let rec1 = Rectangle {
    	width: width1,
    	height: height1,
    };

    let rec2 = Rectangle {
    	width: 10,
    	height: 20,
    };

    let square = Rectangle::square(5);

    println!("The area of the rectangle is {} square pixels.", rec1.area());

    println!("can {:?} fit in {:?}? {}", rec1, rec2, rec1.can_fit(&rec2));

    println!("area of square: {:?}", square.area());
}
