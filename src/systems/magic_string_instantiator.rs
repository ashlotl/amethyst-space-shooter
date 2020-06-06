use amethyst::{
	prelude::{
		Builder,
		World,
		WorldExt,
	},
	core::{

		SystemDesc,
		timing::{
			Time,
		},
		Transform,
	},
	derive::SystemDesc,
	ecs::{
		prelude::{
			LazyUpdate,
			System,
			SystemData,
		},
		Entities,
		Entity,
		Join,
		Read,
		ReadStorage,
		WriteStorage,
	},
	input::{
		InputHandler,
		StringBindings,
	},
	winit::MouseButton,
};

use crate::objs::actor::Actor;
use crate::objs::animate::ship::Ship;
use crate::objs::operators::magic_string::MagicString;

#[derive(SystemDesc)]
pub struct MagicStringInstantiatorSystem {

	prev_selected:Option<Entity>,
	last_mouse_press:f64,
}

static mut world_ref:Option<*mut World>=Option::None;

impl MagicStringInstantiatorSystem {
	pub fn new()->Self {
		Self {
			prev_selected:Option::None,
			last_mouse_press:0.0,
		}
	}
	pub fn set_wf(wf:*mut World) {
		unsafe {
			world_ref=Some(wf);
		}
	}
	pub fn get_wf()->*mut World {
		unsafe {
			world_ref.unwrap()
		}
	}
}

impl Default for MagicStringInstantiatorSystem {
	fn default() -> Self {
		Self::new()
	}
}

impl<'s> System<'s> for MagicStringInstantiatorSystem {
	type SystemData = (
		ReadStorage<'s,Ship>,
		WriteStorage<'s,MagicString>,
		Entities<'s>,
		ReadStorage<'s,Transform>,
		Read<'s,LazyUpdate>,
		Read<'s,InputHandler<StringBindings>>,
		Read<'s,Time>,
	);

	fn run(&mut self, (ships, mstrings, entities, poss, updater, input, time):Self::SystemData) {
		let ct=time.absolute_time_seconds();
		if let Some(mouse_position) = input.mouse_position() {
			if input.mouse_button_is_down(MouseButton::Left)&&ct-self.last_mouse_press>0.2 {
				self.last_mouse_press=ct;
				let mouse_position={
					let mut t = Transform::default();
					t.set_translation_xyz(
						(mouse_position.0-500.0)/10.0,
						-(mouse_position.1-500.0)/10.0,
						0.0,
					).translation().clone()
				};
				for (ship,pos,ent) in (&ships,&poss,&entities).join() {
					let dvec=pos.translation().clone()-mouse_position.clone();
					println !("m:{},p:{}",mouse_position,pos.translation());
					let dvec_mag=dvec.x*dvec.x+dvec.y*dvec.y+dvec.z*dvec.z;
					let r = ship.get_radius_with_transform(pos,&mouse_position);
					let selected=ent;

					if dvec_mag<=r*r{
						println !("I've got a hit");
						if let Some(ps) = self.prev_selected {
							unsafe {
								println !("Wowweeee!");
								let nms = entities.create();
								let dist = {
									let d=(poss.get(ps).unwrap().translation()-poss.get(selected).unwrap().translation());
									(d.x*d.x+d.y*d.y+d.z*d.z).sqrt()
								};
								updater.insert(nms,MagicString {one:ps,two:selected,length:dist,max_tension:1.0});
							}
							self.prev_selected=Option::None;
						} else {
							self.prev_selected=Option::Some(selected);
						}
						break;
					}
				}
			}
		}
	}
}
