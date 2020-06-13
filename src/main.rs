mod util;

use util::model;

fn main() {
	let x = model::Point {
		x: 200,
		y: 300,
	};

	println!("{} {}", x.x, x.y);

}
