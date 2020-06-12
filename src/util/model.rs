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
	pub body: [ [ Option<Color>; 20 ]; 20 ], 
}

pub struct Coord(u8, u8);

pub struct Mass {
	pub body: [ [ Option<Color>; 20 ]; 20 ], 
	pub perimeter: Vec<Coord>,
	perimeter_reference_point: Coord,
}

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

// make sure the coord is not out of bounds
// if it is return None
fn get_coord_neighbor (coord: &Coord, diff: (i16, i16)) -> Option<Coord> {
	let x = coord.0 + diff.0;
	let y = coord.1 + diff.1;
	if x >= 0 && x <= 20 && y >= 0 && y <= 20 {
		Some(Coord(x, y))
	}
	else { None }
}

// just tells us how to move, given the direction
fn get_coord_from_direction (direction: &Dir, coord: &Coord) -> Option<Coord> {
	let next:(i16, i16) = match direction {
		Dir::E => 	(-1, 0),
		Dir::NE => 	(-1, 1),
		Dir::N => 	(0, 1),
		Dir::NW => 	(1, 1),
		Dir::W => 	(1, 0),
		Dir::SW => 	(1, -1),
		Dir::S => 	(0, -1),
		Dir::SE => 	(-1, -1),
	}
	get_coord_neighbor(coord, next)
}

fn get_next_direction (direction: Option<&Dir>) -> Dir {
	match direction {
		Some(dir) => {
			match dir {
				Dir::E => Dir::NE,
				Dir::NE => Dir::N,
				Dir::N => Dir::NW,
				Dir::NW => Dir::W,
				Dir::W => Dir::SW,
				Dir::SW => Dir::S,
				Dir::S => Dir::SE,
				Dir::SE => Dir::E,
			}
		},
		None => Dir::E,
	}
}

fn trace_2d_perimeter (body: [[Option<Color>;20];20], coord: Coord, last_dir: Option<Dir>) -> Vec<Coord> {
	let direction: Dir = get_next_direction(last_dir);

	let mut next_pixel: Option<Pixel> = None;
	let mut found_empty = false;
	while let None = next_coord {
		let (x, y) = get_coord_from_direction(direction, coord);
	}

	vec![]
}

impl Mass {
	fn calculate_perimeter(mut self) {
		self.perimeter = {
			trace_2d_perimeter(self.body, self.perimeter_reference_point, None)
		}
	}
}
