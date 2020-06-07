struct Point {
	x: u16,
	y: u16,
}

fn main() {
	let x = Point {
		x: 200,
		y: 300,
	};
	println!("{} {}", x.x, x.y);
}
