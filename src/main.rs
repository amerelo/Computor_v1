mod list_module;
mod degree2_module;

extern crate regex;

use regex::Regex;
use list_module::module::Module;
use degree2_module::solver::Solver;
use std::env;
use std::process;

fn execut(str: &String)
{
	let re = Regex::new(r"(\+|\-|=)?\s*(\-?\d+\.?\d*)\s*\*\s*[Xx]\^(\-?\d+\.?\d*)\s*").unwrap();
	let mut module = Module::default();
	let mut flag: bool = false;

	for cap in re.captures_iter(str) {
		if let &Some(_m) = &cap[3].find('.') { println!("polynomial not valid"); process::exit(0x0f00); }
		if let &Some(_m) = &cap[3].find('-') { println!("polynomial not valid"); process::exit(0x0f00); }

		if let Some(_m) = cap.get(1) {
			let mut signe: bool = true;
			if &cap[1] == "=" {
				flag = true;
				module.change_list();
			}
			else {
				signe = if &cap[1] == "+" {true} else {false};
			}
			module.add_to_list(signe, (&cap[2]).parse().unwrap(), (&cap[3]).parse().unwrap());
		} else {
			module.add_to_list(true, (&cap[2]).parse().unwrap(), (&cap[3]).parse().unwrap());
		}
	}
	if flag == false || module.lista.is_empty() || module.listb.is_empty() {
		println!("format error");
		process::exit(0x0f00);
	}

	module.true_signe();
	if  module.check_valid_degree() == 0.0 {
		Solver::solve0(&module);
		process::exit(0x0f00);
	}
	module.redu();
	module.regroup();
	module.print_redu();
	// module.print_vec_a();
	// module.print_vec_b();

	let degree = module.check_valid_degree();

	println!("Polynomial degree: {}", degree);
	if  degree > 2.0 {
		println!("The polynomial degree is stricly greater than 2, I can't solve it.");
	} else {
		if degree == 2.0 {
			Solver::solve2(&module);
		}
		else if degree == 1.0 {
			module.redu_1();
			Solver::solve1(&mut module);
		} 
	}
}

fn main()
{
	let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
		execut(&args[1]);
    } else {
		println!("One arg need");
	}
}

// todo 
// - regroupe 2^2 + 1^2 | OK
// - error of - -x | OK
// - reduc print | OK
// - pars 1 degree | OK
// - 1 degree | OK
// - standar entry | OK
// - error ^-1 - ^32.2 - 0^1 | OK

// - ireal number | OK ?
// - pars 0 degree | OK ?
// - 0 degree | OK ?
