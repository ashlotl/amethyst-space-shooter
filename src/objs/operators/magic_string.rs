use amethyst::{

	ecs::{
		Component,
		DenseVecStorage,
		Entity,
		ReadStorage,
		Storage,
	}
};

pub struct MagicString {
	pub one:Entity,
	pub two:Entity,
	pub length:f32,
	pub max_tension:f32,
}

impl Component for MagicString {
	type Storage = DenseVecStorage<Self>;
}
