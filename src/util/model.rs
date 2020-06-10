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
	pub color: Color,
}

let FRAME_SIZE = 20;

pub struct Visible {
	pub body: [ [ Pixel; FRAME_SIZE ]; FRAME_SIZE ], 
}

pub struct Mass {
	pub body: Vec<(Point, Color)>,
	pub perimeter: Vec<Point>,
}

impl Mass {
	fn calculate_perimeter(&self) {
	}
}
