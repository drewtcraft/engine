use std::convert::TryFrom;
use crate::util::model::atomic::{
	Coord, 
	Dir,
	Point,
	Color,
};
use crate::util::constant::SHIP_SIZE;
use crate::util::model::vector::Vector;

type Body = [ [ Option<Color>; SHIP_SIZE ]; SHIP_SIZE];

pub struct Mass {
	pub body: Body, 
	pub anchor: Point,
	pub perimeter: Vec<Coord>,
	pub perimeter_reference_point: Coord,
	pub center: Coord,
	pub vector: Vector,
	pub orientation: Dir,
}

fn get_diff_from_direction (direction: &Dir) -> (i16, i16) {
	match direction {
		Dir::E => 	(-1, 0),
		Dir::NE => 	(-1, 1),
		Dir::N => 	(0, 1),
		Dir::NW => 	(1, 1),
		Dir::W => 	(1, 0),
		Dir::SW => 	(1, -1),
		Dir::S => 	(0, -1),
		Dir::SE => 	(-1, -1),
	}
}

fn get_mass (body: &Body) -> f64 {
	let mut count: f64 = 0.0;
	for a in body.iter() {
		for b in a.iter() {
			match b {
				Some(_) => count += 1.0,
				None => {},
			};
		}
	}
	count
}

impl Mass {
	fn collide (mut self, obj: &Mass) {
		let mass1 = get_mass(&self.body);
		let mass2 = get_mass(&obj.body);
		let vector1 = &self.vector;
		let vector2 = &obj.vector;
		let x = (mass1 * vector1.x) + (mass2 * vector2.x) / mass1 + mass2;
		let y = (mass1 * vector1.y) + (mass2 * vector2.y) / mass1 + mass2;
		self.vector = Vector { x, y };
	}

	// TODO make this more precise, get max X and Y, use their centers
	fn get_center (&self) -> Coord {
			Coord(SHIP_SIZE / 2, SHIP_SIZE / 2)
	}

	fn rotate (mut self, direction: Dir) {
		// first calculate the angle
		let mut angle: f64 = 15.0;
		let mut dir: Dir = get_next_direction(&self.orientation);
		loop {
			match &dir {
				direction => break,
				_ => {
					angle += 15.0;
					dir = get_next_direction(&dir);
				},
			}
		}

		let sin_of_angle = &angle.sin();
		let cos_of_angle = &angle.cos();

		fn rotate_coord (coord: &Coord, sin: &f64, cos: &f64) -> Coord {
			// convert coord to f64
			let fx: f64 = coord.0 as f64;
			let fy: f64 = coord.1 as f64;

			// get new coordinate
			let x = fx * cos - fy * sin;
			let y = fy * cos + fx * sin;

			Coord(x.ceil() as usize, y.ceil() as usize)
		}

		let mut new_body: Body;

		for (x, a) in self.body.iter().enumerate() {
			for (y, color) in a.iter().enumerate() {
				let new_coord = rotate_coord(&Coord(x, y), sin_of_angle, cos_of_angle);
				new_body[new_coord.0][new_coord.1];
			}
		}


	}

	fn reposition (self) {
	}
}


fn inc_dimension (dimension: usize, delta: i16) -> Option<usize> {
	match delta {
		1 => {
			if dimension == SHIP_SIZE { 
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
	let diff: (i16, i16) = get_diff_from_direction(direction);

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

fn trace_2d_perimeter (	body: &Body, 
						coord: &Coord, 
						mut v: Vec<Coord>, 
						last_dir: Option<Dir>) -> Vec<Coord> {

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
