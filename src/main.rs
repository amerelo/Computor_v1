extern crate regex;

use regex::Regex;

fn main()
{
	let re = Regex::new(r"(\d+)\s*\*\s*(\D{1})\^(\d+)").unwrap();

	let test =  "5854 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0";
	// let test =  "21312 hola roro";
	println!("length {}", re.captures_len());
	println!("test {}", re);
	println!("length {}", re.captures_len());
	for cap in re.captures_iter(test) {
		println!("f1: {}, f2: {} f3: {}", &cap[1], &cap[2], &cap[3]);
	}
}
