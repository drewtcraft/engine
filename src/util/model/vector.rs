// so for a vector, we can just store a single coordinate and compare it to
// (0, 0) to get the angle and length (force)
pub struct Vector {
	pub x: f64,
	pub y: f64,
}

impl Vector {
	fn get_length (&self) -> f64 {
		let n = (self.x * self.x) + (self.y * self.y);
		(n as f64).sqrt()
	}

	fn get_angle (&self) -> f64 {
		let deg = self.y.atan2(self.x).to_degrees();
		if deg < 0.0 {
			deg + 360.0
		}
		else {
			deg
		}
	}
}
