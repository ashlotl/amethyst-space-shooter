use amethyst::{
	core::{
		math::{
			base::{
				ArrayStorage,
				Vector3,
			},
			geometry::{
				Translation,
			},
			Matrix,
			U3,
			U4,
		},
		SystemDesc,
		timing::{
			Time,
		},
		Transform,
	},
	derive::{
		SystemDesc,
	},
	ecs::{
		prelude::{
			System,
			SystemData,
		},
		Join,
		Read,
		ReadStorage,
		WriteStorage,
	},
	input::{
		InputHandler,
		StringBindings,
	},
};


use crate::{
	objs::animate::ship::Ship,
	objs::actor::MatrixVel,
};



#[derive(SystemDesc)]
pub struct PlayerInputSystem {
	pub scale:f32,
}


impl<'s> System<'s> for PlayerInputSystem {
	type SystemData = (
		WriteStorage<'s, MatrixVel>,//vel
		WriteStorage<'s, Ship>,
		Read<'s, InputHandler<StringBindings>>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut vels, mut ships, input, time): Self::SystemData) {
		let mut id=0;
		for (vel,ship) in (&mut vels, &mut ships).join() {
			let delta=time.delta_seconds()*self.scale;
			let accel1 = match id {
				0 => {
					let mut t = Transform::default();
					t.set_translation_xyz(
						input.axis_value("first_ship_right").unwrap()*delta*delta,//squaring is intentional
						input.axis_value("first_ship_up").unwrap()*delta*delta,
						0.0,
					);
					t
				},
				1 => {
					let mut t = Transform::default();
					t.set_translation_xyz(
						input.axis_value("second_ship_right").unwrap()*delta*delta,//squaring is intentional
						input.axis_value("second_ship_up").unwrap()*delta*delta,
						0.0,
					);
					t
				},
				_ => {
					Transform::default()//you'd put some code to retrieve from netcode buffer here, or "ai". For now we'll just let it fall endlessly.
				},
			};
			//now, instead of leaving this information for the next system, we'll just add to the position for a test.
			vel.1.prepend_translation(accel1.translation().clone());
			// vel.0.set_translation_xyz(
			// 	//x
			//
			// );

			id+=1;
		}
	}
}
