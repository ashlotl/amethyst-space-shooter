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
		Read<'s, Time>,
		Entities<'s>,
	);

	fn run(&mut self, (mut vels, mut poss, ships, time, entities):Self::SystemData) {
		let delta=time.delta_seconds()*self.scale;
		for id in (&*entities).join() {
			if let Some(ship) = ships.get(id) {
				//immutable borrows first
				let pos = poss.get(id).unwrap();
				let elastic_collision_loss = ship.elastic_loss();
				//apply dv
				let op=pos.translation();
				for (oid) in (&*entities).join() {
					if oid!=id {
						if let Some(ship2) = ships.get(oid) {
							let pos2 = poss.get(oid).unwrap();
							let tp = pos2.translation();
							let op = pos.translation();
							let x_to = tp.x-op.x;
							let y_to = tp.y-op.y;
							let z_to = tp.z-op.z;
							let dist_sqrd:f32 = (x_to*x_to+y_to*y_to+z_to*z_to);
							let dist =  (dist_sqrd).sqrt();
							let o_mass = ship.calculate_mass();
							let t_mass = ship2.calculate_mass();

							// gravity time
							let f = self.grav_const*t_mass*o_mass/dist_sqrd;
							if dist!=0.0 {
								vels.get_mut(id).unwrap().1.prepend_translation({
									let mut t = Transform::default();
									t.set_translation_xyz(
										(tp.x-op.x)/dist*f,
										(tp.y-op.y)/dist*f,
										(tp.z-op.z)/dist*f,
									).translation().clone()
								});
							}

							//collision time
							let t_vel=vels.get(oid).unwrap();
							let t_vtr = t_vel.0.translation();
							let t_vtr_x = t_vtr.x;
							let t_vtr_y = t_vtr.y;
							let t_vtr_z = t_vtr.z;
							let o_vel=vels.get_mut(id).unwrap();
							let o_vtr = o_vel.0.translation();
							let o_vtr_x=o_vtr.x;
							let o_vtr_y=o_vtr.y;
							let o_vtr_z=o_vtr.z;
							if dist <= ship.get_radius_with_transform(pos,tp)+ship2.get_radius_with_transform(pos2,op) && (o_vtr_x.signum()==x_to.signum()&&x_to!=0.0&&o_vtr_x!=0.0 || o_vtr_y.signum()==y_to.signum()&&y_to!=0.0&&o_vtr_y!=0.0 || o_vtr_z.signum()==z_to.signum()&&z_to!=0.0&&o_vtr_z!=0.0) {


								let t_vel_mag_sqrd = t_vtr_x*t_vtr_x+t_vtr_y*t_vtr_y+t_vtr_z*t_vtr_z;
								let t_energy = 0.5*t_mass*t_vel_mag_sqrd-(ship.elastic_loss()+ship2.elastic_loss());

								let xr = x_to/dist;
								let yr = y_to/dist;
								let zr = z_to/dist;

								let x = -(xr*t_energy*2.0/o_mass).abs().sqrt()*x_to.signum();
								let y = -(yr*t_energy*2.0/o_mass).abs().sqrt()*y_to.signum();
								let z = -(zr*t_energy*2.0/o_mass).abs().sqrt()*z_to.signum();

								o_vel.1.prepend_translation({
									let mut t = Transform::default();
									t.set_translation_xyz(
										x,
										y,
										z,
									).translation().clone()
								});
							}
						}
					}
				}
			}
		}
		for (vel,pos) in (&mut vels,&mut poss).join() {
			vel.0=vel.1.clone();
			pos.prepend_translation(vel.0.translation().clone()*delta*self.translation_scale);
			let (xr,yr,zr) = vel.0.euler_angles();
			let (xpr,ypr,zpr) = pos.euler_angles();
			pos.set_rotation_euler(xr*delta*self.rotation_scale+xpr*delta*self.rotation_scale,yr*delta*self.rotation_scale+ypr,zr*delta*self.rotation_scale+zpr);
		}
	}
}
