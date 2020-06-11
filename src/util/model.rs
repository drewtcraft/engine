pub struct Point {
	pub x: u16,
	pub y: u16,
}

pub struct Rect {
	pub top_left: Point,
	pub top_right: Point,
	pub bottom_right: Point,
	pub bottom_left: Point,
}

pub struct Color(u8, u8, u8);

pub struct Pixel {
	pub point: Point,
	pub color: Option<Color>,
}

pub struct Visible {
	pub body: [ [ Pixel; 20 ]; 20 ], 
}

pub struct Mass {
	pub body: [ [ Pixel; 20 ]; 20 ], 
	pub perimeter: Vec<Point>,
	perimeter_reference_point: Point,
}

pub struct Coord(u8, u8);

// direction enum, letters represent cardinal directions
pub enum Dir {
	E,
	NE,
	N,
	NW,
	W,
	SW,
	S,
	SE,
}

fn get_neighbor (direction: Dir, coord: Coord) -> Coord {
	match direction {
		Dir::E => Coord(coord.0 - 1, coord.1),
		Dir::NE => Coord(coord.0 - 1, coord.1 + 1),
		Dir::N => Coord(coord.0, coord.1 + 1),
		Dir::NW => Coord(coord.0 + 1, coord.1 + 1),
		Dir::W => Coord(coord.0 + 1, coord.1),
		Dir::SW => Coord(coord.0 + 1, coord.1 - 1),
		Dir::S => Coord(coord.0, coord.1 - 1),
		Dir::SE => Coord(coord.0 - 1, coord.1 - 1),
	}
}

fn trace_perimeter (body: [[Pixel;20];20], point: Coord, last_coord: Coord) {
	
}

impl Mass {
	fn calculate_perimeter(&self) {
		// so I think if we have a point to start with

		// we look in a clockwise direction at our neighbors
		// when we find a blank
	}
}
