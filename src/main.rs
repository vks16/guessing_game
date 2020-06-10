extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number between 1 to 100.");

	let secret_number = rand::thread_rng().gen_range(1,101);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read");

		let guess: u32 = match guess.trim().parse(){
							Ok(num) => num,
							Err(_) => {
								println!("Enter only a valid number please!\n");
								continue;
							},
						};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too Small"),
			Ordering::Greater => println!("Too Big"),
			Ordering::Equal => {
				println!("You win");
				break;
			},
		}
	}
}
