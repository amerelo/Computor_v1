
use list_module::module::Module;
use list_module::module::Elem;
use std::process;

pub struct Solve2
{
    
}

impl Solve2
{

    pub fn get_value(list: &Vec<Elem>, find: f32) -> f32
    {
        for elem in list {
            if elem.power == find {
                return elem.value;
            }
        }
        process::exit(0x0f00);
    }

    pub fn solve(elem: &Module)
    {
        let a = Solve2::get_value(&elem.lista, 2.0);
        let b = Solve2::get_value(&elem.lista, 1.0);
        let c = Solve2::get_value(&elem.lista, 0.0);

        // println!("a {} b {} c {}", a, b, c);
        let delta = b.powf(2.0) - 4.0 * a * c;
        // println!("delta {}", delta);

        if delta > 0.0 {
            println!("2 solutions !");
            println!("sol 1 = {}", (-b - delta.sqrt()) / (2.0 * a));
            println!("sol 2 = {}", (-b + delta.sqrt()) / (2.0 * a));
        }
        else if delta == 0.0{
            println!("1 solutions !");
            println!("sol = {}", -b / (2.0 * a));
        }
        else {
            println!("0 solutions !");
            process::exit(0x0f00);
        }
    }
}