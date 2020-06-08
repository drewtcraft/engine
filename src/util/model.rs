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
