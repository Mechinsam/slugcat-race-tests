use raylib::prelude::*;
use rayon::prelude::*;

use std::slice;

mod entity;
mod gamestates;
mod assets;
mod map;
mod rendersystem;

use crate::gamestates::GameState; // i dont feel like typing out gamestates::GameState all the time lol
use rendersystem::Viewport;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const MAXFPS: u32 = 75;

pub fn texture_to_collision_mask(texture: &Texture2D, scale: f32) -> Vec<bool>
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
	// Game vars
	let mut game_state = GameState::InRace; // Either "InRace" or "win"
	let slugcats_should_move: bool = true; // Set to 'true' to ignore countdown (not implemented yet)
	let mut show_debug: bool = false;
	let mut winner: String = String::from("/");

	let mut viewport: rendersystem::Viewport = rendersystem::Viewport::init(
		"SRT (idling)",
		SCREEN_WIDTH,
		SCREEN_HEIGHT,
		MAXFPS
	);

	// Load all assets
	// Does not matter what game state you are in
	let map: map::Map = map::Map::new("Blocks", &mut viewport);
	let mut slugcats: Vec<entity::Slugcat> = assets::load_slugcats(&mut viewport, map.gate_spawn_pos);

	let mut win_image: Texture2D = viewport.load_image("DATA/racers/win/_default.png");

	if game_state == GameState::InRace {
		viewport.change_title(&format!("SRT ({})", map.map_name));
	}

	// Game loop
	while !viewport.window.window_should_close() {
		// Input
		// Why Input before Setup? Because 'viewport.window' cant have two mutable references at once ('drawer' needs it as well)
		if viewport.window.is_key_pressed(KeyboardKey::KEY_Q) {
			break; // Breaks gameloop. Apparently you should not call the function to do that because that messes up some things or smth
		}
		if viewport.window.is_key_pressed(KeyboardKey::KEY_D) {
			show_debug = !show_debug;
		}

		
		if game_state == GameState::Win && winner !="/"
		{
			println!("HEY WE LOADED THE WIN TEXTURE IT HAS BEEN REPLACED I PROMISE!!!!!!!!!");
			win_image = viewport.load_image(
				&format!("DATA/racers/win/{}.png", winner)
			);
		}

		// Setup
		let delta_time: f32 = viewport.window.get_frame_time();
		let mouse_pos = &viewport.get_mouse_position();
		let mut drawer: RaylibDrawHandle<'_> = viewport.window.begin_drawing(&viewport.thread);
		drawer.clear_background(Color::BLACK);

		
		if game_state == GameState::InRace
		{
			// Background priority
			drawer.draw_texture(&map.background, 0, 0, Color::WHITE);
			
			// Update and Slugcats
			if slugcats_should_move
			{
				// Multi-threaded update
				slugcats
					.par_iter_mut()
					.for_each(|slugcat: &mut entity::Slugcat| slugcat.update(
						SCREEN_WIDTH,
						SCREEN_HEIGHT,
						delta_time,
						&map.col_map,
						SCREEN_WIDTH,
						SCREEN_HEIGHT
					));
			}

			// Slugcat render
			for slugcat in &slugcats {
				slugcat.draw(&mut drawer);
			}

			// Render food
			winner = map.food.update(&slugcats);
			map.food.draw(&mut drawer);

			if winner != "/"
			{
				println!("{}", winner);
				game_state = GameState::Win;
			}
		}
		else if game_state == GameState::Win
		{
			drawer.draw_texture(&win_image, 0, 0, Color::WHITE);
			drawer.draw_text(&winner, 10, SCREEN_HEIGHT-100, 100, Color::BLACK);
		}
		else
		{
			drawer.draw_text("fin", (SCREEN_WIDTH/2) as i32, (SCREEN_HEIGHT/2) as i32, 24, Color::WHITE);
		}

		if show_debug
		{
			// Render FPS 
			drawer.draw_fps(0, 0);

			// Display mouse position
			let mouse_pos_text: String = format!("MOUSE POS: ({}, {})", mouse_pos.x, mouse_pos.y);

			drawer.draw_text(&mouse_pos_text, 0, 20, 20, Color::LIGHTGREEN);
		}
	}
}
