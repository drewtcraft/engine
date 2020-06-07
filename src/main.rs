use std::fmt;

struct Point {
	pub x: u16,
	pub y: u16,
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, " pt({}, {}) ", self.x, self.y)
	}
}

fn main() {
	let x = Point {
		x: 200,
		y: 300,
	};
	println!("{}", x);
}
