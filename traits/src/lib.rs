use std::fmt;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	pub fn eat<T:Food>(&mut self, food: T) {
		self.strength += food.gives();
	}
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        4.0 * self.weight_in_kg
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let fat_weight = self.fat_content * self.weight_in_kg;
        let prot_weight = (1.0 - self.fat_content) * self.weight_in_kg;
        (9.0 * fat_weight) + (4.0 * prot_weight)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.name);
        write!(f, "Strength: {}, Score: {}, Money: {}\n", self.strength, self.score, self.money);
        write!(f, "Weapons: {:?}", self.weapons)
    }
}