mod ecs;
mod atomic;

use ecs::{ entity, component, system };


fn main() {
	system::drift(&entity::Entity::new(), 1);
}
