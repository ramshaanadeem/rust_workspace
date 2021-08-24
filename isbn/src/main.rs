use std::io;
use std::io::prelude::*;

fn validate(isbn: &String) {
	// println!("in validate");
	let mut sum = 0;
	let mut j = 10;
	let mut _valid: bool;

	for digits in isbn.chars() {
		if digits == '-' {
			continue;
		} else if digits == 'X' {
			sum += 10;
			break;
		}
		if j == 0 {
			break;
		}
		let digit: u32 = (digits as u32) - 48;
		sum += j * digit;
		j -= 1;
	}

	if sum % 11 == 0 {
		_valid = true;
		println!("Valid isbn")
	} else {
		println!("Invalid isbn")
	}
}

fn main() {
	let mut isbn = String::new();
	print!("Enter ISBN: ");
	std::io::stdout()
		.flush()
		.ok()
		.expect("Could not flush stdout");
	io::stdin()
		.read_line(&mut isbn)
		.expect("Failed to read string");
	if isbn.len() - 1 == 10 || isbn.len() - 1 == 13 {
		validate(&isbn);
	}
}
