use crate::resource_management;
use crate::objs::*;

use amethyst::{
	prelude::{
		SimpleState,
		StateData,
		GameData,
		WorldExt,
	},
	assets::{
		Handle,
	},
	renderer::{
		SpriteSheet,
	},
};

pub const NUM_SHIPS:u32 = 50;

pub const GAME_WIDTH:f32 = 100.0;
pub const GAME_HEIGHT:f32 = 100.0;

pub const PI:f32 = 3.14159;

#[derive(Default)]
pub struct OrbFire {
	sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for OrbFire {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		self.sprite_sheet_handle.replace(resource_management::make_sprite_sheet(world));

		world.register::<animate::ship::Ship>();
		world.register::<special::camera::CameraOptions>();

		for id in 0..NUM_SHIPS {
			animate::ship::init_ship(world, self.sprite_sheet_handle.clone().unwrap(), id);
		}

		special::camera::init_camera(world);
	}
}
