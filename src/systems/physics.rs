use amethyst::{
	core::{
		math::{
			geometry::{
				Translation3,
			},
			Vector3
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
		Entities,
		Join,
		Read,
		ReadStorage,
		WriteStorage,
	},
};

use crate::{
	game::{
		GAME_WIDTH,
		GAME_HEIGHT,
	},
	objs::{
		actor::{
			Actor,
			MatrixVel,
		},
		animate::{
			ship::{
				Ship,
			},
		},
		operators::{
			magic_string::MagicString,
		},
	},
};

#[derive(SystemDesc)]
pub struct PhysicsSystem {
	pub scale:f32,
	pub translation_scale:f32,
	pub rotation_scale:f32,
	pub gravity_scale:f32,
	pub grav_const:f32,
}

impl<'s> System<'s> for PhysicsSystem {
	type SystemData = (
		WriteStorage<'s, MatrixVel>,
		WriteStorage<'s, Transform>,
		ReadStorage<'s, Ship>,
		WriteStorage<'s, MagicString>,
		Read<'s, Time>,
		Entities<'s>,
	);

	fn run(&mut self, (mut vels, mut poss, ships, mut mstrings, time, entities):Self::SystemData) {
		let delta=time.delta_seconds()*self.scale;

		unsafe {
			static mut IT:u64=0;
			if time.absolute_time_seconds() as u64/5==IT {
				println !("d:{}",delta);
				IT+=1;
			}
		}
		for id in (&*entities).join() {
			if let Some(ship) = ships.get(id) {
				//immutable borrows first
				let pos=poss.get(id).unwrap().clone();
				let elastic_collision_loss = ship.elastic_loss();
				//apply dv
				let op=pos.translation();

				for (oid) in (&*entities).join() {
					if oid!=id {
						if let Some(ship2) = ships.get(oid) {
							let pos2 = poss.get(oid).unwrap();
							let tp = pos2.translation();
							let x_to = tp.x-op.x;
							let y_to = tp.y-op.y;
							let z_to = tp.z-op.z;
							let dist_sqrd:f32 = (x_to*x_to+y_to*y_to+z_to*z_to);

							let dist =  (dist_sqrd).sqrt();

							let xr = x_to/dist;
							let yr = y_to/dist;
							let zr = z_to/dist;

							let o_mass = ship.calculate_mass();
							let t_mass = ship2.calculate_mass();

							// gravity time
							let f = self.grav_const*t_mass/dist_sqrd;
							if dist!=0.0 {
								vels.get_mut(id).unwrap().1.prepend_translation({
									let mut t = Transform::default();
									t.set_translation_xyz(
										xr*f,
										yr*f,
										zr*f,
									).translation().clone()
								});
							}

							//collision time
							let t_vel=&vels.get(oid).unwrap().0;
							let t_vtr = t_vel.translation();
							let t_vtr_x = t_vtr.x;
							let t_vtr_y = t_vtr.y;
							let t_vtr_z = t_vtr.z;
							let o_vel=vels.get_mut(id).unwrap();
							let o_vtr = o_vel.0.translation();
							let o_vtr_x=o_vtr.x;
							let o_vtr_y=o_vtr.y;
							let o_vtr_z=o_vtr.z;
							if dist <= ship.get_radius_with_transform(&pos,tp)+ship2.get_radius_with_transform(pos2,op) && (o_vtr_x*xr+o_vtr_y*yr+o_vtr_z*zr)>0.0 {


								let coeff_1=(o_mass-t_mass)/(t_mass+o_mass);
								let coeff_2=2.0*t_mass/(t_mass+o_mass);

								let x=t_vtr_x;
								let y=t_vtr_y;
								let z=t_vtr_z;

								let omag=(o_vtr_x*o_vtr_x+o_vtr_y*o_vtr_y+o_vtr_z*o_vtr_z).sqrt()*coeff_1;

								let rmag=(x*x+y*y+z*z).sqrt()*coeff_2+omag;

								o_vel.1.prepend_translation(
									{
										let mut t=Transform::default();
										t.set_translation_xyz(
											-rmag*xr,
											-rmag*yr,
											-rmag*zr,
										);
										t.translation().clone()
									}
								);
							}
						}
					}
				}

				for (ent) in (&*entities).join() {
					if let Some(mstring) = mstrings.get(ent) {
						// let mstring=mstrings.get(ent).unwrap().clone();
						let op=poss.get(mstring.one).unwrap();
						let opt=op.translation();

						let tp=poss.get(mstring.two).unwrap();
						let tpt=tp.translation();

						let x_to = tpt.x-opt.x;
						let y_to = tpt.y-opt.y;
						let z_to = tpt.z-opt.z;


						let dist=(x_to*x_to+y_to*y_to+z_to*z_to).sqrt();


						let xr=x_to/dist;
						let yr=y_to/dist;
						let zr=z_to/dist;

						let oship=ships.get(mstring.one).unwrap();
						let tship=ships.get(mstring.two).unwrap();
						// en garde
						let o_mass=oship.calculate_mass();
						let t_mass=tship.calculate_mass();

						let o_vel=vels.get(mstring.one).unwrap();
						let o_vtr=o_vel.0.translation();
						let o_vtr_x=o_vtr.x;
						let o_vtr_y=o_vtr.y;
						let o_vtr_z=o_vtr.z;

						let t_vel=vels.get(mstring.two).unwrap();
						let t_vtr=o_vel.0.translation();
						let t_vtr_x=t_vtr.x;
						let t_vtr_y=t_vtr.y;
						let t_vtr_z=t_vtr.z;

						let string_distance = mstring.length;
						let max_tension = mstring.max_tension;
						if dist>string_distance {
							let coeff_1=(o_mass-t_mass)/(t_mass+o_mass);
							let coeff_2=2.0*t_mass/(t_mass+o_mass);

							let x=t_vtr_x;
							let y=t_vtr_y;
							let z=t_vtr_z;

							let omag=(o_vtr_x*o_vtr_x+o_vtr_y*o_vtr_y+o_vtr_z*o_vtr_z).sqrt()*coeff_1;
							// println !("En garde!");
							let rmag=(x*x+y*y+z*z).sqrt()*coeff_2+omag;
							if o_mass*omag/coeff_1>max_tension {
								let sub=max_tension/o_mass;
								vels.get_mut(mstring.one).unwrap().1.prepend_translation(
									{
										let mut t=Transform::default();
										t.set_translation_xyz(
											sub*xr,
											sub*yr,
											sub*zr,
										);
										t.translation().clone()
									}
								);
								vels.get_mut(mstring.two).unwrap().1.prepend_translation(
									{
										let mut t=Transform::default();
										t.set_translation_xyz(
											-sub*xr,
											-sub*yr,
											-sub*zr,
										);
										t.translation().clone()
									}
								);
								entities.delete(ent);
								mstrings.remove(ent);
							} else {
								vels.get_mut(mstring.one).unwrap().1.prepend_translation(
									{
										let mut t=Transform::default();
										t.set_translation_xyz(
											rmag*xr,
											rmag*yr,
											rmag*zr,
										);
										t.translation().clone()
									}
								);
								vels.get_mut(mstring.two).unwrap().1.prepend_translation(
									{
										let mut t=Transform::default();
										t.set_translation_xyz(
											-rmag*xr,
											-rmag*yr,
											-rmag*zr,
										);
										t.translation().clone()
									}
								);
							}
						}
					}
				}

				//finish up by ensuring everything is on-screen
				let o_vel=vels.get_mut(id).unwrap();
				let o_vtr = o_vel.0.translation();
				let o_vtr_x=o_vtr.x;
				let o_vtr_y=o_vtr.y;
				let o_vtr_z=o_vtr.z;
				let mut tm=vels.get_mut(id).unwrap().1.translation_mut();

				if op.x>=GAME_WIDTH/2.0 && o_vtr_x>0.0 {
					tm.x*=-1.0;
				} else if op.x<=-GAME_WIDTH/2.0 && o_vtr_x<0.0 {
					tm.x*=-1.0;
				}
				if op.y>=GAME_HEIGHT/2.0 && o_vtr_y>0.0 {
					tm.y*=-1.0;
				} else if op.y<=-GAME_HEIGHT/2.0 && o_vtr_y<0.0 {
					tm.y*=-1.0;
				}

			}
		}

		//strings
		// //this system works in "reverse" -- that is, each object affects others and not itself.
		//
		// for (id) in (&*entities).join() {
		// 	if let Some(ship) = ships.get(id) {
		// 		for (oid) in (&*entities).join() {
		// 			if oid!=id {
		// 				if let Some(ship2) = ships.get(oid) {
		//
		// 				}
		// 			}
		// 		}
		// 	}
		// }

		for (vel,pos) in (&mut vels,&mut poss).join() {
			vel.0=vel.1.clone();
			pos.prepend_translation(vel.0.translation().clone()*delta*self.translation_scale);
			let (xr,yr,zr) = vel.0.euler_angles();
			let (xpr,ypr,zpr) = pos.euler_angles();
			pos.set_rotation_euler(xr*delta*self.rotation_scale+xpr*delta*self.rotation_scale,yr*delta*self.rotation_scale+ypr,zr*delta*self.rotation_scale+zpr);
		}
	}
}
