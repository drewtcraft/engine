use std::convert::TryFrom;
use crate::util::model::mass::Mass;
use crate::util::model::atomic::Point;
use crate::util::model::atomic::Coord;
use crate::util::model::atomic::Dir;
use std::f64::consts::PI;


pub enum Weapon {
	Missile,
	Laser,
}

pub struct Cannon {
	pub weapon_type: Weapon,
	pub position: Coord,
	pub relative_orientation: Dir,
}

pub struct Ship {
	pub mass: Mass,
	pub orientation: Dir,
	pub cannons: [ Option<Cannon>; 8 ],
	pub engines: [ Option<Coord>; 20 ],
}

enum Turn {
	Left,
	Right,
}

impl Ship {
	fn intersects (&self, point: &Point) -> bool {
		let start = &self.mass.anchor;

		for c in self.mass.perimeter.iter() {
			let x = u16::try_from(c.0).unwrap();
			let y = u16::try_from(c.1).unwrap();
			let is_equal = {
				point.x == x && point.y == y
			};
			if is_equal { return true; }
		}

		false
	}
}
