use std::io;

fn main() {
	let mut first_number = String::new();
	let mut second_number = String::new();

	println!("Enter two numbers: ");

	io::stdin()
		.read_line(&mut first_number)
		.expect("Failed to read number");

	let mut first_number: u32 = match first_number.trim().parse() {
		Ok(first_number) => first_number,
		Err(_) => {
			println!("Error");
			0
		}
	};

	io::stdin()
		.read_line(&mut second_number)
		.expect("Failed to read number");

	let mut second_number: u32 = match second_number.trim().parse() {
		Ok(second_number) => second_number,
		Err(_) => {
			println!("Error");
			0
		}
	};

	while first_number != second_number {
		if first_number > second_number {
			first_number -= second_number
		} else {
			second_number -= first_number
		}
	}

	println!("Greatest Common divisor is: {} ", first_number);
}
