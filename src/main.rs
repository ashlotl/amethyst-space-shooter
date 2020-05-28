//lonely
mod game;
mod resource_management;

//have children
mod objs;
mod systems;
mod utility;

//what matters in this file
use crate :: {
	game	:: OrbFire,
	systems	:: {
		magic_string_instantiator::MagicStringInstantiatorSystem,
		player_input	:: PlayerInputSystem,
		physics			:: PhysicsSystem,
	},
};

use amethyst :: {
    core :: transform :: TransformBundle,
    prelude :: *,
	input :: {
		InputBundle,
		StringBindings,
	},
    renderer :: {
        plugins :: {
			RenderFlat2D,
			RenderToWindow
		},
        types :: DefaultBackend,
        RenderingBundle,
    },
    utils :: application_root_dir,
};

fn main() -> amethyst :: Result<()> {
    amethyst::start_logger(Default :: default());

    let app_root = application_root_dir()?;

    let assets_dir 			= app_root. 	join("assets");

    let config_dir 			= app_root. 	join("config");
    let display_config_path	= config_dir.	join("display.ron");
	let binding_path		= config_dir.	join("bindings.ron");

	let inb = InputBundle::<StringBindings>::new()
		.with_bindings_from_file(binding_path)?;

	unsafe {
		let mut msis = MagicStringInstantiatorSystem::new();
		let refe = &mut msis as *mut MagicStringInstantiatorSystem;

	    let game_data = GameDataBuilder :: default()
	        .with_bundle(
	            RenderingBundle :: <DefaultBackend> :: new()
	                .with_plugin(
	                    RenderToWindow :: from_config_path(display_config_path)?
	                        .with_clear([0.0, 0.1, 0.7, 1.0]),
	                )
	                .with_plugin(
						RenderFlat2D :: default()
					),
	        )?
	        .with_bundle(TransformBundle :: new())?
			.with_bundle(
				inb,
			)?
			.with(
				PlayerInputSystem {
					scale:5.0,
				},
				"player_input_system",
				&[],
			)
			.with(
				msis,
				"magic_string_instantiator_system",
				&[],
			)
			.with(
				PhysicsSystem {
					scale:100.0,
					translation_scale:1.0,
					rotation_scale:1.0,
					gravity_scale:10.0,
					grav_const:0.01,
				},
				"physics_system",
				&["player_input_system"],
			);

	    let mut game = Application :: new(assets_dir, OrbFire::new(Option::None,refe), game_data)?;
	    game.run();
	}
    Ok(())
}
