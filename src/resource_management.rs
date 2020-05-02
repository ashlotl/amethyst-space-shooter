use amethyst::{
	prelude::{
		World,
		WorldExt,
	},
	assets::{
		AssetStorage,
		Handle,
		Loader,
	},
	renderer::{
		ImageFormat,
		SpriteSheet,
		SpriteSheetFormat,
		Texture,
	},
};

pub fn make_sprite_sheet(world:&mut World) -> Handle<SpriteSheet> {
	let texture_handle = load_texture_handle(world, "texture/sprite_sheet.png");
	load_sprite_sheet(world, "texture/ron/sprite_sheet.ron", texture_handle)
}

fn load_texture_handle(world: &mut World, path: &str) ->  Handle<Texture> {
	let loader = world.read_resource::<Loader>();
	let texture_storage = world.read_resource::<AssetStorage<Texture>>();
	loader.load(
		path,
		ImageFormat::default(),
		(),
		&texture_storage,
	)
}

fn load_sprite_sheet(world: &mut World, path: &str, texture_handle:Handle<Texture>) -> Handle<SpriteSheet> {
	let loader = world.read_resource::<Loader>();
	let sprite_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load(
		path,
		SpriteSheetFormat(texture_handle),
		(),
		&sprite_storage,
	)
}
