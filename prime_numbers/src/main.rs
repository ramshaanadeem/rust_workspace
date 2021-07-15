// find prime numbers in an interval
use std::io;

fn main() {
	println!("Enter two numbers (Interval): ");

	let mut lower_limit = String::new();
	let mut upper_limit = String::new();

	io::stdin().read_line(&mut lower_limit).expect("Error");

	let mut lower_limit: u32 = match lower_limit.trim().parse() {
		Ok(lower_limit) => lower_limit,
		Err(_) => {
			println!("Error");
			0
		}
	};

	io::stdin().read_line(&mut upper_limit).expect("Error");

	let upper_limit: u32 = match upper_limit.trim().parse() {
		Ok(upper_limit) => upper_limit,
		Err(_) => {
			println!("Error");
			0
		}
	};

	let mut _prime: bool = true;

	println!("Prime Numbers: ");
	while upper_limit > lower_limit {
		_prime = true;

		if lower_limit == 0 || lower_limit == 1 {
			_prime = false;
		} else {
			for i in 2..lower_limit / 2 + 1 {
				// println!("i = {}",i);

				if lower_limit % i == 0 {
					_prime = false;
					break;
				}
			}
		}

		if _prime {
			println!("{}", lower_limit)
		}

		lower_limit += 1
	}
}
