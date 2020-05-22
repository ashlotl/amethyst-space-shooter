//helper types for your actors to be stored in parallel to your actor components, and some methods all actors must have.
use std::ops;

use amethyst::{
	core::{
		math::{
			base::{
				ArrayStorage,
				Matrix,
			},
			geometry::Translation3,
			U1,
			U3,
			U4,
		},
		Transform,
	},
	ecs::prelude::{
		Component,
		DenseVecStorage,
	},
};




pub trait Actor {
	fn take_damage<T: Actor>(&mut self,amount:u32, dam_type:DamageType, from_id:&T);
	fn do_collision<T: Actor>(&mut self,input_newtons:f32,other:&T);
	fn calculate_mass(&self) -> f32;

	fn get_radius_with_transform(&self, my_transform:&Transform, translation:&Matrix<f32,U3,U1,ArrayStorage<f32,U3,U1>>) -> f32 {//get radius from transform (of this ship) to translation (other collider), taking into account rotation and shape of sprite
		0.1
	}

	fn elastic_loss(&self) -> f32 {
		0.3
	}

	fn to_string (&self) -> String {
		String::from("(unimplemented to_string) Actor")
	}
}

//types
pub struct MatrixVel(pub Transform,pub Transform);

impl Component for MatrixVel {
	type Storage=DenseVecStorage<Self>;
}


pub enum DamageType {
	Ballistic,
	Fire,
	Gamma,
	Yeet,
}
