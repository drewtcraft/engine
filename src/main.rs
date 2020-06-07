mod util;

use util::structs;

fn main() {
	let x = structs::Point {
		x: 200,
		y: 300,
	};
	println!("{} {}", x.x, x.y);
}
