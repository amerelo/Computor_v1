
use list_module::module::Module;
use list_module::module::Elem;
use std::process;

pub struct Solver
{

}

impl Solver
{
	pub fn get_value(list: &Vec<Elem>, find: f32) -> f32
	{
		for elem in list {
			if elem.power == find {
				return elem.value;
			}
		}
		println!("format error");
		process::exit(0x0f00);
	}

	pub fn solve0(elem: &Module)
	{
		let var1 = elem.lista.iter().fold(0.0,|acc, v| acc + v.value);
		let var2 = elem.listb.iter().fold(0.0,|acc, v| acc + v.value);

		if var1 == var2 {
			println!("{} * X^0 = {} * X^0 Every real number is a solution. \nOne of many solutions is: 42", var1, var2);
		} else {
			println!("{} * X^0 = {} * X^0 Ther is no solution", var1, var2);
		}
	}

	pub fn solve1(elem: &mut Module)
	{
		if !elem.listb.is_empty() && !elem.lista.is_empty() && elem.listb[0].value != 0.0 {
			elem.lista[0].value = elem.lista[0].value / elem.listb[0].value;
			elem.listb[0].value = 1.0;
		}

		if !elem.lista.is_empty() {
			println!("The solution is: {}", elem.lista[0].value);
		} else {
			println!("The solution is: 0");
		}
	}

	pub fn solve2(elem: &Module)
	{
		let a = Solver::get_value(&elem.lista, 2.0);
		let b = Solver::get_value(&elem.lista, 1.0);
		let c = Solver::get_value(&elem.lista, 0.0);

		let delta = b.powf(2.0) - 4.0 * a * c;

		if delta > 0.0 {
			println!("Discriminant is strictly positive, 2 solutions are :");
			println!("sol 1 = {}", (-b - delta.sqrt()) / (2.0 * a));
			println!("sol 2 = {}", (-b + delta.sqrt()) / (2.0 * a));
		}
		else if delta == 0.0{
			println!("Discriminant is null, 1 solutions are :");
			println!("sol = {}", -b / (2.0 * a));
		}
		else {
			println!("Discriminant is strictly negatif, 2 complexes solutions are :");
			println!("sol 1 = ({} + i * sqrt({})) / (2 * {})", -b, -delta, a);
			println!("sol 2 = ({} - i * sqrt({})) / (2 * {})", -b, -delta, a);
			process::exit(0x0f00);
		}
	}
}
