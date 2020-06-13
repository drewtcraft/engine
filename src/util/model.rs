pub struct Point {
	pub x: u16,
	pub y: u16,
}

pub struct Color(u8, u8, u8);

pub struct Coord(usize, usize);

// direction enum, letters represent cardinal directions
pub enum Dir { E, NE, N, NW, W, SW, S, SE }

pub struct Mass {
	pub body: [ [ Option<Color>; 20 ]; 20 ], 
	pub perimeter: Vec<Coord>,
	pub anchor: Point,
	pub perimeter_reference_point: Coord,
}

fn inc_dimension (dimension: usize, delta: i16) -> Option<usize> {
	match delta {
		1 => {
			if dimension == 20 { 
				Some(dimension + 1) 
			} else { None }
		},
		0 => Some(dimension),
		-1 => {
			if dimension == 0 { 
				Some(dimension - 1)
			} else { None }
		},
		_ => None
	}
}

// translates a direction + starting position into a new coord
// or if we are out of bounds, None
fn get_2d_neighbor (coord: &Coord, direction: &Dir) -> Option<Coord> {
	let diff: (i16, i16) = match direction {
		Dir::E => 	(-1, 0),
		Dir::NE => 	(-1, 1),
		Dir::N => 	(0, 1),
		Dir::NW => 	(1, 1),
		Dir::W => 	(1, 0),
		Dir::SW => 	(1, -1),
		Dir::S => 	(0, -1),
		Dir::SE => 	(-1, -1),
	};

	// should definitely abstract this bit into a function
	let x: Option<usize> = inc_dimension(coord.0, diff.0);
	let y: Option<usize> = inc_dimension(coord.1, diff.1); 
	match x {
		Some(x) => {
			match y {
				Some(y) => Some(Coord(x, y)),
				None => None,
			}
		},
		None => None,
	}
}

// returns next cardinal direction moving clockwise
fn get_next_direction (direction: &Dir) -> Dir {
	match direction {
		Dir::E => Dir::NE,
		Dir::NE => Dir::N,
		Dir::N => Dir::NW,
		Dir::NW => Dir::W,
		Dir::W => Dir::SW,
		Dir::SW => Dir::S,
		Dir::S => Dir::SE,
		Dir::SE => Dir::E,
	}
}

fn is_diagonal (direction: &Dir) -> bool {
	match &direction{
		Dir::NE => true,
		Dir::NW => true,
		Dir::SW => true,
		Dir::SE => true,
		_ => false,
	}
}

fn coords_are_equal (a: Option<Coord>, b: &Coord) -> bool {
	match a {
		Some(a) => {
			a.0 == b.0 && a.1 == b.1
		},
		None => false,
	}
}

fn trace_2d_perimeter (body: &[[Option<Color>;20];20], coord: &Coord, mut v: Vec<Coord>, last_dir: Option<Dir>) -> Vec<Coord> {
	// get next direction from last direction;
	// use East by default (for now)
	let direction: Dir = match last_dir {
		Some(d) => get_next_direction(&d),
		None => Dir::E
	};

	let mut next_coord: Option<Coord> = None;
	let mut hit_empty = false; // track if we have hit an empty
	while let None = next_coord {
		next_coord = get_2d_neighbor(coord, &direction);
		match &next_coord {
			Some(c) => { // coord is in-bounds

				// is the Color empty?
				if let None = body[c.0][c.1] {
					hit_empty = true;
				} else { // the Color is not empty
					if hit_empty {

						// for diagonals we want to make sure the NEXT
						// clockwise coord is also not empty
						if is_diagonal(&direction) {
							// get next direction
							let dirr = get_next_direction(&direction);
							// check next coord
							match get_2d_neighbor(&c, &dirr) {
								// if we have the "anchor" pixel, push to v
								Some(_) => v.push(Coord(c.0, c.1)),

								// if it is empty, invalidate our current coord
								None => hit_empty = false,
							}
						}
						// non-diagonals are good to push
						else { v.push(Coord(c.0, c.1)) }
					}
				}
			},
			None => { // coord is out of bounds
				// we can treat this as an empty
				hit_empty = true;
			},
		}
		
	}

	if coords_are_equal(next_coord, coord) { v }
	else { trace_2d_perimeter(body, coord, v, Some(direction)) }
}

impl Mass {
	fn calculate_perimeter(mut self) {
		self.perimeter = {
			trace_2d_perimeter(&self.body, &self.perimeter_reference_point, Vec::new(), None)
		}
	}
}
