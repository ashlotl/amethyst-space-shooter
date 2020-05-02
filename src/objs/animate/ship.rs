use amethyst::{
	prelude::{
		World,
	},
	assets::{
		Handle,
	},
	renderer::{
		SpriteSheet,
	},
};

use crate::{
	game::{
		GAME_WIDTH,
		GAME_HEIGHT,
		PI,
	}
}


pub struct Ship {
	health: u32,
	ammo: i32,
	fuel: f32,
}

const DEF_SHIP_HEALTH:u32 = 100;
const DEF_SHIP_AMMO:i32 = 13;
const DEF_SHIP_FUEL:f32 = 100.0;

pub fn init_ship(world:&mut World, sprite_handle:Handle<SpriteSheet>, id:u32) {
	let mut local_transform = Transform::default();

	let angle = id/NUM_SHIPS*2*PI;
	let x=GAME_WIDTH*angle.cos();
	let y=GAME_HEIGHT*angle.sin();
	local_transform.set_translation_xyz(x, y, 0);


	let mut velocity = Transform::default();

	velocity.set_translation_xyz(-y, x, 0);


	let sprite_render = SpriteRender {
		sprite_sheet: sprite_handle,
		sprite_number: 0,
	}

	world.create_entity()
		.with(sprite_render)
		.with(Ship {
			health: DEF_SHIP_HEALTH,
			ammo: DEF_SHIP_AMMO,
			fuel: DEF_SHIP_FUEL,
		})
		.with(local_transform)
		.with(velocity)
		.build();
}
