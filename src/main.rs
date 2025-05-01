use raylib::prelude::*;
use rendersystem::Viewport;
use std::{slice, fs};

mod entity;
mod map;
mod rendersystem;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const MAXFPS: u32 = 75;
const DRAWFPS: bool = true;

fn load_racers(viewport: &mut Viewport) -> Vec<entity::Slugcat>
{
	let dir: &str = "DATA/racers/sprites";
	let entries: fs::ReadDir = fs::read_dir(dir).expect(&format!("Failed to read {}", dir));

	// get slugcats from directory
	let slugcat_textures: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            // If extension == "png", extract file_name as String; else skip
            path.extension()
                .and_then(|ext| ext.to_str())
                .filter(|&ext| ext.eq_ignore_ascii_case("png"))
                .and_then(|_| path.file_name()                                           
                    .and_then(|os| os.to_str())
                    .map(|s| s.to_owned()))
        })
        .collect();

	let mut slugcats: Vec<entity::Slugcat> = Vec::new();

	let mut counter: i32 = 1;
	let slugcats_spacing: i32 = 10; // Distance between slugcats. Kind of like... padding

	for racer_texture_path in slugcat_textures
	{
		let racer_texture_path: String = format!("DATA/racers/sprites/{}", racer_texture_path);

		let mut slugcat: entity::Slugcat = entity::Slugcat::new(viewport.load_image(&racer_texture_path), 0.25);
		let width: i32 = slugcat.texture.width() + slugcats_spacing;

		for i in 0..counter {
			slugcat.position.x += width as f32 * slugcat.scale;
		}
		

		slugcats.push(slugcat);

		counter += 1;
	}
	
	return slugcats;
}

fn texture_to_collision_mask(texture: &Texture2D, scale: f32) -> Vec<bool>
{
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
		"SRT",
		SCREEN_WIDTH,
		SCREEN_HEIGHT,
		MAXFPS
	);

	// Load all assets
	let map: map::Map = map::Map::new("map1", &mut viewport);
	let mut slugcats: Vec<entity::Slugcat> = load_racers(&mut viewport);

	viewport.change_title(&format!("SRT ({})", map.map_name));

	// Game loop
	while !viewport.window.window_should_close() {
		// Setup
		let delta_time: f32 = viewport.window.get_frame_time();
		let mut drawer: RaylibDrawHandle<'_> = viewport.window.begin_drawing(&viewport.thread);
		drawer.clear_background(Color::BLACK);

		// Background priority
		drawer.draw_texture(&map.background, 0, 0, Color::WHITE);
		
		// Render Slugcats
		for racer in &mut slugcats
		{
			racer.update(SCREEN_WIDTH, SCREEN_HEIGHT, delta_time, &map.col_map, SCREEN_WIDTH, SCREEN_HEIGHT);
			racer.draw(&mut drawer);
		}

		// Render food
		map.draw_food(&mut drawer);

		// Render FPS if needed
		if DRAWFPS {
			drawer.draw_fps(0, 0);
		}
	}
}
