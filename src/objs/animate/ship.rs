use amethyst::{
	prelude::{
		Builder,
		World,
		WorldExt,
	},
	assets::{
		Handle,
	},
	ecs::{
		Component,
		DenseVecStorage,
	},
	core::{
		Transform,
	},
	renderer::{
		SpriteRender,
		SpriteSheet,
	},
};
use crate::{
	game::{
		GAME_WIDTH,
		GAME_HEIGHT,
		NUM_SHIPS,
		PI,
	},
	objs::{
		actor::{
			Actor,
			DamageType,
		},
	},
};


pub struct Ship {
	health: u32,
	ammo: i32,
	fuel: f32,
}

impl Component for Ship {
	type Storage = DenseVecStorage<Self>;
}

impl Actor for Ship {
	fn take_damage<T: Actor>(&mut self,amount:u32,_type:DamageType,from:&T) {
		if amount<=self.health {
			self.health-=amount;
		} else {
			self.health = 0;
		}
		println !("The meanie {} hurt me!", from.to_string());
	}

	fn do_collision<T: Actor>(&mut self, input_newtons:f32, other:&T) {
		println !("Ouch.");
	}

	fn to_string(&self) -> String {
		String::from(format!("Ship: health:{} ammo:{} fuel:{}",self.health, self.ammo, self.fuel))
	}
}

const DEF_SHIP_HEALTH:u32 = 100;
const DEF_SHIP_AMMO:i32 = 13;
const DEF_SHIP_FUEL:f32 = 100.0;

pub fn init_ship(world:&mut World, sprite_handle:Handle<SpriteSheet>, id:u32) {


	let angle = 0.2 * id as f32/NUM_SHIPS as f32 * PI;

	let x=GAME_WIDTH*angle.cos()/4.0;
	let y=GAME_HEIGHT*angle.sin()/4.0;

	let mut velocity = Transform::default();

	velocity.set_translation_xyz(-y, x, 0.0);


	let mut local_transform = Transform::default();

	local_transform.set_translation_xyz(x, y, 0.0);


	let sprite_render = SpriteRender {
		sprite_sheet: sprite_handle,
		sprite_number: 0,
	};

	world.create_entity()
		.with(sprite_render)
		.with(Ship {
			health: DEF_SHIP_HEALTH,
			ammo: DEF_SHIP_AMMO,
			fuel: DEF_SHIP_FUEL,
		})
		.with(velocity)
		.with(local_transform)//the order here is crucial!! amethyst will use the latter for position!
		.build();
}
