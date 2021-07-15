use std::io;

fn main() {
	let mut length = String::new();

	println!("Enter the length of sequence: ");
	io::stdin().read_line(&mut length).expect("Failed");

	let length: u32 = match length.trim().parse() {
		Ok(length) => length,
		Err(_) => 0,
	};

	let mut previous_number = 0;
	let mut current_number = 1;
	let mut next_item = current_number + previous_number;

	println!("{} ", previous_number);
	println!("{} ", current_number);

	for _element in 1..length + 1 {
		println!("{} ", next_item);
		previous_number = current_number;
		current_number = next_item;
		next_item = previous_number + current_number;
	}
}
