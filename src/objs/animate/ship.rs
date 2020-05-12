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
		math::{
			geometry::Translation,
			Matrix,
			U3,
		},
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
	utility::{
		transform_math,
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

	fn exists_collision<T: Actor>(&mut self, other:&mut T, loc1:Transform, loc2:Transform, collision_layers_1:[i32;2], collision_layers_2:[i32;2]) -> bool {
		if collision_layers_1[1]>=collision_layers_2[0]||collision_layers_1[0]<=collision_layers_2[1] {
			let tr2 = loc2.translation().clone();
			let tr1 = loc1.translation().clone();
			// let vec:Vector = Vector::from(tr2.x-tr1.x,tr2.y-tr1.y,tr2.z-tr1.z);
			if transform_math::dist_sqrd(tr1,tr2)<=(self.get_radius_with_transform(loc1,Translation::<f32,U3>::from(tr2-tr1))+other.get_radius_with_transform(loc2,Translation::<f32,U3>::from(tr1-tr2))).powi(2) {
				return true;
			}
		}
		false
	}

	fn to_string(&self) -> String {
		String::from(format!("Ship:  health:{} ammo:{} fuel:{}",self.health, self.ammo, self.fuel))
	}
}

const DEF_SHIP_HEALTH:u32 = 100;
const DEF_SHIP_AMMO:i32 = 13;
const DEF_SHIP_FUEL:f32 = 100.0;

pub fn init_ship(world:&mut World, sprite_handle:Handle<SpriteSheet>, id:u32) {


	let angle = 2.0/NUM_SHIPS as f32 * id as f32 * PI;

	let x=GAME_WIDTH*angle.cos()/4.0;
	let y=GAME_HEIGHT*angle.sin()/4.0;

	let mut velocity = Matrix::new();

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
