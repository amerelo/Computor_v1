extern crate regex;

use regex::Regex;

fn main()
{
	let re = Regex::new(r"(\d+)\s*\*\s*(\D{1})\^(\d+)\s*(\+|\-|=)?\s*").unwrap();

	let test =  "5854 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0";
	// let test =  "21312 hola roro";
	println!("length {}", re.captures_len());
	println!("test {}", re);
	println!("length {}", re.captures_len());
	for cap in re.captures_iter(test) {
		// println!("New line ");
		// for elem in cap.iter() {
		// 	match elem 
		// 	{
		// 		Some(x) => {
		// 			println!("ok {:?}", x);
		// 			// println!("f1: {}, f2: {} f3: {},f4: {}", &cap[1], &cap[2], &cap[3], x);
		// 		}
		// 		None => {
		// 			println!("Error");
		// 			// println!("f1: {}, f2: {} f3: {}", &cap[1], &cap[2], &cap[3]);
		// 		}
		// 	}
		// }

		if let Some(_m) = cap.get(4) {
			println!("f1: {}, f2: {}, f3: {}, f4: {}", &cap[1], &cap[2], &cap[3], &cap[4]);
		}
		else {
			println!("f1: {}, f2: {} f3: {}", &cap[1], &cap[2], &cap[3]);
		}
	}
}
