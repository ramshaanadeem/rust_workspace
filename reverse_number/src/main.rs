use std::io;

fn main() {
	let mut number = String::new();

	println!("Enter Number to be reversed: ");
	io::stdin().read_line(&mut number).expect("Error");

	let mut number: u32 = match number.trim().parse() {
		Ok(number) => number,
		Err(_) => {
			println!("Error");
			0
		}
	};

	let mut reversed_number: u32 = 0;
	let mut remainder: u32;

	while number != 0 {
		remainder = number % 10;
		reversed_number = reversed_number * 10 + remainder;
		number /= 10;
	}

	println!("Reversed Number: {} ", reversed_number);
}
