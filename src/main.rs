mod list_module;
mod degree2_module;

extern crate regex;

use regex::Regex;
use list_module::module::Module;
use degree2_module::solve2::Solve2;

fn main()
{
	let re = Regex::new(r"(\+|\-|=)?\s*(\-?\d+\.?\d*)\s*\*\s*[Xx]\^(\-?\d+\.?\d*)\s*").unwrap();

	let test = "- -5 * X^0 + -9 * X^1 + 2 * X^2 = 0 * X^0";
	// let test = "+ -58.54 * X^0 ";
	let mut module = Module::default();
	
	for cap in re.captures_iter(test) {
		if let Some(_m) = cap.get(3) {
		//  (&cap[1] == "+" && (&cap[2]).parse().unwrap() > 0 || (&cap[1] == "-" && (&cap[2]).parse().unwrap() < 0)) {
			// if (=)
			let mut signe: bool = true;
			if &cap[1] == "=" {
				module.change_list();
			}
			else {
				signe = if &cap[1] == "+" {true} else {false};
			}
			module.add_to_list(signe, (&cap[2]).parse().unwrap(), (&cap[3]).parse().unwrap());
		} else {
			module.add_to_list(true, (&cap[1]).parse().unwrap(), (&cap[2]).parse().unwrap());
		}
	}

	module.redu();
	
	if module.check_valid_degree() > 2.0 {
		println!("The polynomial degree is stricly greater than 2, I can't solve it.");
	} else {
		println!("solve");
	}
	module.print_vec_a();
	module.print_vec_b();
	
	Solve2::solve(&module);
}