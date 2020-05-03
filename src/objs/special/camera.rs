use amethyst::{
	prelude::{
		Builder,
		World,
		WorldExt,
	},
	core::{
		Transform,
	},
	ecs::prelude::{
		Component,
		DenseVecStorage,
	},
	renderer::{
		Camera,
	},
};

use crate::{
	game::{
		GAME_WIDTH,
		GAME_HEIGHT,
	},
};

enum CameraTranslationMode {
	Unlocked,
	Free(usize),//the id of the transform belonging to a mouse pointer or so
	Lock(usize),//the id of a transform being your player avatar
}

pub struct CameraOptions {
	translation_mode:CameraTranslationMode,
}

impl Component for CameraOptions {
	type Storage = DenseVecStorage<CameraOptions>;//There'll only be one anyways
}

pub fn init_camera(world:&mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
		.with(transform)
		.with(CameraOptions {
			translation_mode:CameraTranslationMode::Unlocked,
		})
		.build();
}
