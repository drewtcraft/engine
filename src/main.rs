mod util;

use util::model;

fn main() {
	let x = model::Point {
		x: 200,
		y: 300,
	};

	println!("{} {}", x.x, x.y);

	fn make_point (x: u16, y: u16) -> model::Point {
		model::Point { x, y }
	}


	let y = model::Rect {
		top_left: make_point(1, 1),
		top_right: make_point(2, 1),
		bottom_right: make_point(2, 0),
		bottom_left: make_point(1, 0),
	};

	println!("{}", y.bottom_left.x);
}
