use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1..101);
	loop {
		println!("Make a guess! ");

		println!("Input: ");
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(guess) => guess,
			Err(_) => continue,
		};

		println!("You guessed: {} \n", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too Small\n"),
			Ordering::Greater => println!("Too Big\n"),
			Ordering::Equal => {
				println!("YOU WIN");
				break;
			}
		}
	}
}
