

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
                println!("elem {:?}", tmp);
            }
        }
    }

    pub fn check_valid_degree(&self) -> f32
    {
        let mut max_power: f32 = 0.0;

        for elem in self.lista.iter() {
            if max_power < elem.power {
                max_power = elem.power;
            }
        }
        return max_power;
    }

    pub fn print_vec_a(&self)
    {
        println!("{:?}", self.lista);
    }

    pub fn print_vec_b(&self)
    {
        println!("{:?}", self.listb);
    }
}