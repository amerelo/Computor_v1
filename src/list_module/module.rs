
use std::process;

#[derive(Debug)]
pub struct Elem
{
	pub signe: bool,
	pub value: f32,
	pub power: f32,
}

#[derive(Debug)]
pub struct Module
{
	pub state: bool,
	pub lista: Vec<Elem>,
	pub listb: Vec<Elem>,
}

impl Module
{
	pub fn default() -> Module
	{
		Module {
			state: false,
			lista: vec![],
			listb: vec![],
		}
	}

	pub fn change_list(&mut self)
	{
		self.state = true;
	}

	pub fn add_to_list(&mut self, signe: bool, value: f32, power: f32)
	{
		if value == 0.0 {
			println!("value not valid {}", value);
			process::exit(0x0f00);
		}

		if self.state == false {
			self.lista.push(Elem {signe: signe, value: value, power: power});
		}
		else {
			self.listb.push(Elem {signe: signe, value: value, power: power});
		}
	}

	pub fn mut_lista(&mut self, signe: bool, value: f32, power: f32)
	{
		for elem in self.lista.iter_mut()
		{
			if power == elem.power {
				elem.value -= value;
				return ;
			}
		}
		self.lista.push(Elem {signe: signe, value: value * -1.0, power: power});
	}

	pub fn redu(&mut self)
	{
		while self.listb.is_empty() == false {
			if let Some(tmp) = self.listb.pop() {
				self.mut_lista(tmp.signe, tmp.value, tmp.power);
				// println!("elem {:?}", tmp);
			}
		}
	}

	pub fn redu_1(&mut self)
	{
		let mut len = self.lista.len();
		let mut i = 0;

		while i < len
		{
			if self.lista[i].power == 1.0 {
				let mut tmp = self.lista.remove(i);
				tmp.value = tmp.value * -1.0;
				self.listb.push(tmp);
				len = len - 1;
			} else {
				i = i + 1;
			}
		}
	}

	pub fn true_signe(&mut self)
	{
		for elem in self.lista.iter_mut()
		{
			if (elem.value > 0.0 && elem.signe == false) || (elem.value < 0.0 && elem.signe == false)
			{
				elem.value = elem.value * -1.0;
			}
		}

		for elem in self.listb.iter_mut()
		{
			if (elem.value > 0.0 && elem.signe == false) || (elem.value < 0.0 && elem.signe == false)
			{
				elem.value = elem.value * -1.0;
			}
		}
	}

	fn regroup_elem(&mut self, i: f32) -> Elem
	{
		let mut tmp: Elem = Elem{signe: true, value: 0.0, power: i};

		for elem in self.lista.iter() {
			if i == elem.power {
				tmp.value = tmp.value + elem.value;
			}
		}
		return tmp;
	}

	fn get_max_power(&self) -> f32
	{
		let mut i: f32 = -1.0;
		for elem in self.lista.iter()
		{
			if i < elem.power {
				i = elem.power;
			}
		}
		i = i + 1.0;
		return i;
	}

	pub fn regroup(&mut self)
	{
		let mut i = 0.0;
		let mut new_list: Vec<Elem> = vec![];
		let max_power = self.get_max_power();

		while i < max_power {
			new_list.push(self.regroup_elem(i));
			i = i + 1.0;
		}
		new_list.retain(|elem| elem.value != 0.0);
		self.lista = new_list;
	}

	pub fn check_valid_degree(&self) -> f32
	{
		let mut max_power: f32 = 0.0;

		for elem in self.lista.iter() {
			if max_power < elem.power {
				max_power = elem.power;
			}
		}
		for elem in self.listb.iter() {
			if max_power < elem.power {
				max_power = elem.power;
			}
		}
		return max_power;
	}

	pub fn print_redu(&self)
	{
		print!("Reduced form: ");
		for (i, elem) in self.lista.iter().enumerate() {
			if elem.value > 0.0 && i != 0 { print!("+ "); }
			print!("{} X^{} ", elem.value, elem.power);
		}
		println!("= 0");
	}

	// pub fn print_vec_a(&self)
	// {
	//	 println!("{:?}", self.lista);
	// }

	// pub fn print_vec_b(&self)
	// {
	//	 println!("{:?}", self.listb);
	// }
}
