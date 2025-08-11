

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" => Ok(Antigen::A),
			"B" => Ok(Antigen::B),
			"AB" => Ok(Antigen::AB),
			"O" => Ok(Antigen::O),
			_ => Err(format!("Invalid antigen: {}", s)),
		}
	}
}

impl FromStr for RhFactor {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(RhFactor::Positive),
			"-" => Ok(RhFactor::Negative),
			_ => Err(format!("Invalid Rh factor: {}", s)),
		}
	}
}

impl Ord for BloodType {
	fn cmp(&self, other: &Self) -> Ordering {
		self.antigen.cmp(&other.antigen).then(self.rh_factor.cmp(&other.rh_factor))
	}
}

impl FromStr for BloodType {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (antigen_str, rh_str) = if s.ends_with('+') {
			(&s[..s.len() - 1], "+")
		} else if s.ends_with("-") {
			(&s[..s.len() - 1], "-")
		} else {
			return Err(format!("Invalid blood type format: {}", s));
		};

		let antigen = antigen_str.parse::<Antigen>().unwrap();
		let rh_factor = rh_str.parse::<RhFactor>().unwrap();

		Ok(BloodType { antigen, rh_factor })
	}
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let antigen = match &self.antigen {
			Antigen::A => "A",
			Antigen::B => "B",
			Antigen::AB => "AB",
			Antigen::O => "O",
		};
		let rh_factor = match &self.rh_factor {
			RhFactor::Positive => "+",
			RhFactor::Negative => "-",
		};
		write!(f, "{}{}", antigen, rh_factor)
	}
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
		let antigen = match (&self.antigen, &other.antigen) {
			(Antigen::AB, _) => true,
			(Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
			(Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
			(Antigen::O, Antigen::O) => true,
			_ => false,
		};
		
		let rh = match (&self.rh_factor, &other.rh_factor) {
			(RhFactor::Positive, _) => true,
			(RhFactor::Negative, RhFactor::Negative) => true,
			(RhFactor::Negative, RhFactor::Positive) => false,
		};

		antigen && rh
	}

	pub fn donors(&self) -> Vec<Self> {
		let res = vec![
			BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
			BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
			BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
			BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
		];

		res.into_iter().filter(|a| self.can_receive_from(a)).collect()
	}

	pub fn recipients(&self) -> Vec<BloodType> {
		let res = vec![
			BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
			BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
			BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
			BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
			BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
		];

		res.into_iter().filter(|a| a.can_receive_from(self)).collect()
	}
}