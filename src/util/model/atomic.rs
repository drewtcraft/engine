pub struct Point {
	pub x: u16,
	pub y: u16,
}

pub struct Color(u8, u8, u8);

pub struct Coord(pub usize, pub usize);

// direction enum, letters represent cardinal directions
pub enum Dir { E, NE, N, NW, W, SW, S, SE }

pub struct Momentum {
	pub angle: Dir,
	pub speed: u8,
}
