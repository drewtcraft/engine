use super::component::mass::Mass;
use super::component::vector::Vector;
use uuid::Uuid;

pub struct Entity {
	pub id: String, // generate a hashmap from this
	pub mass: Option<Mass>,
	pub vector: Option<Vector>,
}

impl Entity {
	pub fn new () -> Entity {
		Entity {
			id: Uuid::new_v4().to_string(),
			mass: None,
			vector: None,
		}
	}
}
