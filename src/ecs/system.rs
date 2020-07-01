use crate::ecs::entity::Entity;
use crate::atomic::Point;

//fn collide (mut self, obj: &Mass) {
//	let mass1 = get_mass(&self.body);
//	let mass2 = get_mass(&obj.body);
//	let vector1 = &self.vector;
//	let vector2 = &obj.vector;
//	let x = (mass1 * vector1.x) + (mass2 * vector2.x) / mass1 + mass2;
//	let y = (mass1 * vector1.y) + (mass2 * vector2.y) / mass1 + mass2;
//	self.vector = Vector { x, y };
//}

pub fn drift (ent: &Entity, iteration: usize) {
	if let None = &ent.mass { return };
	if let None = &ent.vector { return };

	let fiteration = iteration as f64;
	let vector = &ent.vector.unwrap();
	let anchor = &ent.mass.as_ref().unwrap().anchor;
	let x = {
		let n = vector.x * fiteration;
		let n = n.floor() as u16;
		anchor.x + n
	};
	let y = {
		let n = vector.y * fiteration;
		let n = n.floor() as u16;
		anchor.y + n
	};
	
	// use our vector to increment our anchor
	let anchor = Point { x, y };
	println!("something running");
}
