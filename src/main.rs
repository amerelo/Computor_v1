extern crate rand;
extern crate regex;

use std::io;
use regex::Regex;
// use std::cmp::Ordering;
// use rand::Rng;
//  3 * X^0 + 1 * X^1 = 0
// -1 * X^0 - 2 * X^1 = 1 * X^0 + 2 * X^1

// struct Sec {
// 	num: i32,
// 	pow: i32,
// }

fn main() {

	println!("Entrez une equation: ");

	let mut equa = String::new();
	io::stdin().read_line(&mut equa).expect("Failed to read line");

	let re = Regex::new(r"\s+").unwrap();
	let result = re.replace_all(equa.as_str().trim(), " ");
	let results = result.split(" ");

	for letter in results{
		println!("|{}|", letter);
		let num = letter.parse::<i32>();
	    match num {
	        Ok(val) => println!("Yes, it is a number ({})", val),
	        Err(why) => println!("Doesn't look like a number ({})", why),
	    }
	}
	println!("|{}|", result);

}


// loop {
// 	println!("Please input your guess.");
//
// 	let guess: u32 = match guess.trim().parse() {
// 		Ok(num) => num,
// 		Err(_) => {
// 			println!("Please type a nbr.");
// 			continue;
// 		}
// 	};
//
// 	println!("You guessed: {}", guess);
//
// 	match guess.cmp(&secret_number)
// 	{
// 		Ordering::Less    => println!("Too small!"),
// 		Ordering::Greater => println!("Too big!"),
// 		Ordering::Equal   => {
// 			println!("You win!");
// 			break;
// 		}
// 	}
// }
