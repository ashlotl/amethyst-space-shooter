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
			MatrixPosition,
			MatrixVel,
		},
		animate::{
			ship::{
				Ship,
			},
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
		WriteStorage<'s, MatrixPosition>,
		ReadStorage<'s, Ship>,
		Read<'s, Time>,
		Entities<'s>,
	);

	fn run(&mut self, (mut vels, mut poss, mut tempposs, ships, time, entities):Self::SystemData) {
		let delta=time.delta_seconds()*self.scale;

		unsafe {
			static mut IT:u64=0;
			if time.absolute_time_seconds() as u64/5==IT {
				println !("d:{}",delta);
				IT+=1;
			}
		}
		for iteration in 0..2 {
			for id in (&*entities).join() {
				if let Some(ship) = ships.get(id) {
					//immutable borrows first
					let pos;
					if iteration==0 {
						pos=poss.get(id).unwrap().clone();
					} else {
						pos=tempposs.get(id).unwrap().0.clone();
					}
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
								let o_mass = ship.calculate_mass();
								let t_mass = ship2.calculate_mass();

								// gravity time
								let f = self.grav_const*t_mass/dist_sqrd;
								if dist!=0.0 {
									vels.get_mut(id).unwrap().c[iteration].prepend_translation({
										let mut t = Transform::default();
										t.set_translation_xyz(
											(x_to)/dist*f,
											(y_to)/dist*f,
											(z_to)/dist*f,
										).translation().clone()
									});
								}

								//collision time
								let t_vel=&vels.get(oid).unwrap().c[2];
								let t_vtr = t_vel.translation();
								let t_vtr_x = t_vtr.x;
								let t_vtr_y = t_vtr.y;
								let t_vtr_z = t_vtr.z;
								let o_vel=vels.get_mut(id).unwrap();
								let o_vtr = o_vel.c[2].translation();
								let o_vtr_x=o_vtr.x;
								let o_vtr_y=o_vtr.y;
								let o_vtr_z=o_vtr.z;
								if dist <= ship.get_radius_with_transform(&pos,tp)+ship2.get_radius_with_transform(pos2,op) && (o_vtr_x.signum()==x_to.signum()&&o_vtr_y.signum()==y_to.signum()&&o_vtr_z.signum()==z_to.signum()) {

									let current_mag = (o_vtr_x*o_vtr_x+o_vtr_y*o_vtr_y+o_vtr_z*o_vtr_z).sqrt();
									let other_mag = (t_vtr_x*t_vtr_x+t_vtr_y*t_vtr_y+t_vtr_z*t_vtr_z).sqrt();

									let coeff_1=(o_mass-t_mass)/(t_mass+o_mass);
									let coeff_2=2.0*t_mass/(t_mass+o_mass);

									let mag=coeff_1*current_mag+coeff_2*other_mag;

									let x = -x_to/dist*mag;
									let y = -y_to/dist*mag;
									let z = -z_to/dist*mag;

									o_vel.c[iteration].prepend_translation(
										{
										let mut t=Transform::default();
										t.set_translation_xyz(
											x,y,z
										);
										t.translation().clone()
										}
									);
								}
							}
						}
					}
					//finish up by ensuring everything is on-screen

					let mut tm=vels.get_mut(id).unwrap().c[iteration].translation_mut();

					if op.x>=GAME_WIDTH/2.0 && tm.x>0.0 {
						tm.x*=-1.0;
					} else if op.x<=-GAME_WIDTH/2.0 && tm.x<0.0 {
						tm.x*=-1.0;
					}
					if op.y>=GAME_HEIGHT/2.0 && tm.y>0.0 {
						tm.y*=-1.0;
					} else if op.y<=-GAME_HEIGHT/2.0 && tm.y<0.0 {
						tm.y*=-1.0;
					}

				}
			}
			if iteration==1 {
				for (vel,pos,temppos) in (&mut vels,&mut poss,&mut tempposs).join() {
					vel.c[2]=vel.c[1].clone();
					let tr1=vel.c[1].translation();
					let tr2=vel.c[2].translation();
					vel.c[2].set_translation((tr1+tr2)/2.0);//VERLET
					pos.prepend_translation(vel.c[2].translation().clone()*delta*self.translation_scale);
					temppos.0=pos.clone();
					let (xr,yr,zr) = vel.c[2].euler_angles();
					let (xpr,ypr,zpr) = pos.euler_angles();
					pos.set_rotation_euler(xr*delta*self.rotation_scale+xpr*delta*self.rotation_scale,yr*delta*self.rotation_scale+ypr,zr*delta*self.rotation_scale+zpr);
				}
			} else {
				for (vel,temppos) in (&mut vels,&mut tempposs).join() {
					vel.c[1]=vel.c[0].clone();
					temppos.0.prepend_translation(vel.c[1].translation().clone()*delta*self.translation_scale);
					let (xr,yr,zr) = vel.c[1].euler_angles();
					let (xpr,ypr,zpr) = temppos.0.euler_angles();
					temppos.0.set_rotation_euler(xr*delta*self.rotation_scale+xpr*delta*self.rotation_scale,yr*delta*self.rotation_scale+ypr,zr*delta*self.rotation_scale+zpr);
				}
			}
		}
	}
}
