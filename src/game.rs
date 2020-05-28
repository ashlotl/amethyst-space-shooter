use crate::resource_management;
use crate::objs::*;
use crate::systems::magic_string_instantiator::MagicStringInstantiatorSystem;

use amethyst::{
	prelude::{
		GameData,
		SimpleState,
		StateData,
		World,
		WorldExt,
	},
	assets::{
		Handle,
	},
	renderer::{
		SpriteSheet,
	},
};

pub const NUM_SHIPS:u32 = 2;

pub const GAME_WIDTH:f32 = 100.0;
pub const GAME_HEIGHT:f32 = 100.0;

pub const PI:f32 = 3.14159;

pub struct OrbFire {
	pub sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl OrbFire {
	pub fn new(ssh:Option<Handle<SpriteSheet>>,_msis:*mut MagicStringInstantiatorSystem) -> Self {
		unsafe {
			msis=Some(_msis);
			Self {
				sprite_sheet_handle:ssh,
			}
		}
	}
}

static mut msis:Option<*const MagicStringInstantiatorSystem>=Option::None;

impl SimpleState for OrbFire {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		unsafe {
			MagicStringInstantiatorSystem::set_wf(data.world as *mut World);
			self.sprite_sheet_handle.replace(resource_management::make_sprite_sheet(&mut *MagicStringInstantiatorSystem::get_wf()));

			(&mut *MagicStringInstantiatorSystem::get_wf()).register::<animate::ship::Ship>();
			(&mut *MagicStringInstantiatorSystem::get_wf()).register::<operators::magic_string::MagicString>();
			(&mut *MagicStringInstantiatorSystem::get_wf()).register::<special::camera::CameraOptions>();

			for id in 0..NUM_SHIPS {
				animate::ship::init_ship(&mut *MagicStringInstantiatorSystem::get_wf(), self.sprite_sheet_handle.clone().unwrap(), id);
			}

			special::camera::init_camera(&mut *MagicStringInstantiatorSystem::get_wf());
		}
	}
}
