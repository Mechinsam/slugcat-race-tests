use raylib::prelude::*;
use std::slice;

mod entity;
mod rendersystem;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const MAXFPS: u32 = 75;
const DRAWFPS: bool = true;


fn texture_to_collision_mask(texture: &Texture2D, scale: f32) -> Vec<bool> {
	let mut image: Image = texture
		.load_image()
		.expect("Failed to read image");

	image.resize((texture.width() as f32 * scale) as i32, (texture.height() as f32 * scale) as i32);
	
	// calculate total pixel count
	let total_px = (image.width * image.height) as usize;
	
	// interpret `data` pointer as color
	let colors: &[raylib::ffi::Color] = unsafe {
		slice::from_raw_parts(
			image.data as *const raylib::ffi::Color,
			total_px,
		)
	};
	
	// make a simple bitmask (opaque pixels (alpha < 25%) = collision)
	colors.iter()
		  .map(|c| c.a > 64) // 64/255 is like almost 25%
		  .collect()
}

fn main()
{
	//let mut delta_time: f32 = 1f32/(MAXFPS as f32);
	let mut viewport: rendersystem::Viewport = rendersystem::Viewport::init(
		"srt64",
		SCREEN_WIDTH,
		SCREEN_HEIGHT,
		MAXFPS
	);
	
	let mut specimen: entity::Entity = entity::Entity::new(viewport.load_image("DATA/slugcat1.png"), 0.5);
	let map_image: Texture2D = viewport.load_image("DATA/map1.png");
	let map: Vec<bool> = texture_to_collision_mask(&map_image, 1f32);

	while !viewport.window.window_should_close() {
		let delta_time: f32 = viewport.window.get_frame_time();
		let mut drawer: RaylibDrawHandle<'_> = viewport.window.begin_drawing(&viewport.thread);

		drawer.clear_background(Color::BLACK);

		specimen.update(SCREEN_WIDTH, SCREEN_HEIGHT, delta_time, &map, SCREEN_WIDTH, SCREEN_HEIGHT);

		//drawer.draw_text("I LOVE SKIBIDI TOILET.", 24, 24, 48, Color::MAROON);
		specimen.draw(&mut drawer);
		drawer.draw_texture(&map_image, 0, 0, Color::WHITE);
		
		//drawer.draw_texture(&spec, 100, 100, Color::WHITE);

		if DRAWFPS {
			drawer.draw_fps(0, 0);
		}
	}
}
