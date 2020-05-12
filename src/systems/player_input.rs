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
pub struct PlayerInputSystem;


impl<'s> System<'s> for PlayerInputSystem {
	type SystemData = (
		WriteStorage<'s, MatrixVel>,//vel
		WriteStorage<'s, Transform>,//pos
		ReadStorage<'s, Ship>,
		Read<'s, InputHandler<StringBindings>>,
		Read<'s, Time>,
	);

	fn run(&mut self, (mut vels, mut poss, ships, input, time): Self::SystemData) {
		let mut id=0;
		for (vel,pos,ship) in (&mut vels,&mut poss, &ships).join() {
			let delta=time.delta_seconds()*4.0;
			let accel1 = match id {
				0 => {
					Vector3::<f32>::new(
						input.axis_value("first_ship_right").unwrap()*delta,
						input.axis_value("first_ship_up").unwrap()*delta,
						0.0,
					)
				},
				1 => {
					Vector3::<f32>::new(
						input.axis_value("second_ship_right").unwrap()*delta,
						input.axis_value("second_ship_up").unwrap()*delta,
						0.0,
					)
				},
				_ => {
					Vector3::<f32>::new(0.0,0.0,0.0)//you'd put some code to retrieve from netcode buffer here, or "ai". For now we'll just let it fall endlessly.
				},
			};
			//now, instead of leaving this information for the next system, we'll just add to the position for a test.
			pos.set_translation(pos.translation().clone()+accel1);

			id+=1;
		}
	}
}
