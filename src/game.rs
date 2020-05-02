use crate::resource_management;
use crate::objs::ship;


use amethyst::{
	prelude::{
		SimpleState,
		StateData,
		GameData,
	},
	assets::{
		Handle,
	},
	renderer::{
		SpriteSheet,
	},
};

const NUM_SHIPS:u32 = 2;

const GAME_WIDTH:f32 = 500.0;
const GAME_HEIGHT:f32 = 500.0;

const PI:f32 = 3.14159;

#[derive(Default)]
pub struct OrbFire {
	sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for OrbFire {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		self.sprite_sheet_handle.replace(resource_management::make_sprite_sheet(world));

		world.register::<Ship>()
		for id in 0..NUM_SHIPS {
			ship::init_ship(world, self.sprite_sheet_handle.clone().unwrap(), id);
		}
	}
}
