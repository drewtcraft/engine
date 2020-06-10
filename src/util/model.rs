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
}

impl Mass {
	fn calculate_perimeter(&self) {
		// so I think if we have a point to start with

		// we look in a clockwise direction at our neighbors
		// when we find a blank
	}
}
