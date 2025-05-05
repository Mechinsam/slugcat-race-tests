use enums::GameEvent;
use raylib::prelude::*;
use rayon::prelude::*;

use std::slice;

mod entity;
mod enums;
mod assets;
mod map;
mod utils;
mod rendersystem;
mod timer;

use crate::enums::GameState; // i dont feel like typing out gamestates::GameState all the time lol
use rendersystem::Viewport;
use timer::Timer;

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
	let audio_sys = RaylibAudio::init_audio_device().expect("Failed to init audio");
	// Game vars
	let mut game_state = GameState::InRace; // Either "InRace" or "win"
	let mut slugcats_should_move: bool = false; // Set to 'true' to ignore countdown (not implemented yet)
	let mut show_debug: bool = false;
	let mut winner: String = String::from("/");
	let mut race_won: bool = false;

	// Timers
	let mut race_timer: Timer = Timer::new();

	race_timer.set(3f32);

	
	let mut viewport: rendersystem::Viewport = rendersystem::Viewport::init(
		"SRT (idling)",
		SCREEN_WIDTH,
		SCREEN_HEIGHT,
		MAXFPS
	);

	// Music & Sound
	let race_track_path = assets::get_music_name(GameState::InRace);
	let win_track_path: String = assets::get_music_name(GameState::Win);

	let mut race_track = audio_sys.new_music(&race_track_path).expect("Failed to load race music!");
	let mut win_track = audio_sys.new_music(&win_track_path).expect("Failed to load win music!");

	let win_sfx = audio_sys.new_sound("DATA/sfx/win.wav").expect("Failed to load win SFX!");
	let applause_sfx = audio_sys.new_sound("DATA/sfx/applause.wav").expect("Failed to load applause SFX!");
	let countdown_sfx = audio_sys.new_sound("DATA/sfx/countdown.wav").expect("Failed to load countdown SFX!");

	race_track.looping = true;
	win_track.looping = true;

	// Load other assets
	// Does not matter what game state you are in, they will be loaded
	let map_name: String = assets::get_map_name_from_file();
	let map: map::Map = map::Map::new(&map_name, &mut viewport);
	let mut slugcats: Vec<entity::Slugcat> = assets::load_slugcats(&mut viewport, map.gate_spawn_pos);

	let mut win_image: Texture2D = viewport.load_image("DATA/racers/win/_default.png");

	if game_state == GameState::InRace {
		viewport.change_title(&format!("SRT ({})", map.map_name));
	}

	countdown_sfx.play();

	let mut event: GameEvent = GameEvent::None;

	// Game loop
	while !viewport.window.window_should_close() {
		// Setup
		let delta_time: f32 = viewport.window.get_frame_time();
		let mouse_pos = &viewport.get_mouse_position();

		// Timers & Music tick
		race_timer.tick(delta_time);

		race_track.update_stream();
		win_track.update_stream();

		// Input
		// Why Input before Setup? Because 'viewport.window' cant have two mutable references at once ('drawer' needs it as well)
		if viewport.window.is_key_pressed(KeyboardKey::KEY_Q) {
			break; // Breaks gameloop. Apparently you should not call the function to do that because that messes up some things or smth
		}
		if viewport.window.is_key_pressed(KeyboardKey::KEY_D) {
			show_debug = !show_debug;
		}
		if viewport.window.is_key_pressed(KeyboardKey::KEY_R) {
			event = GameEvent::RaceWon;
			race_won = true;
		}

		// Events.... I mean.... theres only one
		match &event
		{
			GameEvent::RaceWon =>
			{
				slugcats_should_move = false;

				race_track.stop_stream();
				
				// Check if slugcat-specific win image exists. if not, use the default one
				win_image = if std::path::Path::new(&format!("DATA/racers/win/{}.png", winner)).exists() {
					viewport.load_image(&format!("DATA/racers/win/{}.png", winner))
				} else {
					viewport.load_image("DATA/racers/win/_default.png")
				};

				win_sfx.play();
			}
			GameEvent::UnleashSlugcats =>
			{
				slugcats_should_move = true;
				race_track.play_stream();
			}
			GameEvent::None =>
			{
				// do nothing
			}
		}
		// Clear event
		event = GameEvent::None;

		// Drawer setup
		let mut drawer: RaylibDrawHandle<'_> = viewport.window.begin_drawing(&viewport.thread);
		drawer.clear_background(Color::BLACK);

		match &game_state {
			GameState::InRace =>
			{
				if !win_sfx.is_playing() && race_won
				{
					win_track.play_stream();
					applause_sfx.play();
					game_state = GameState::Win;
				}

				// Render background first
				drawer.draw_texture(&map.background, 0, 0, Color::WHITE);
				
				// Update and Slugcats
				let snapshot: Vec<_> = slugcats
					.iter()
					.map(|sc| sc.entity.to_collision_data(&sc.name))
					.collect();

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
							&snapshot
						));
				}
			
				// Slugcat render
				for slugcat in &slugcats {
					slugcat.draw(&mut drawer);
				}
			
				// Update & render food
				if !race_won
				{
					let check_winner = map.food.update(&slugcats);
					if check_winner != "/"
					{
						winner = check_winner;
						event = GameEvent::RaceWon;
						race_won = true;
					}
				}
				
				map.food.draw(&mut drawer);

				// Draw timer
				if race_timer.seconds_remaining() > 0f32
				{
					drawer.draw_text(&format!("PLACE YOUR BETS! {:.2}", race_timer.seconds_remaining()),
						100,
						(SCREEN_HEIGHT/2) as i32,
						48,
						Color::WHITE
					);
				}
				else
				{
					if !slugcats_should_move && !race_won {
						event = GameEvent::UnleashSlugcats;
					}	
				}
			}
			GameState::Win =>
			{
				drawer.draw_texture(&win_image, 0, 0, Color::WHITE);
				drawer.draw_text(&winner, 10, SCREEN_HEIGHT-100, 100, Color::BLACK);
			}
			_other =>
			{
				drawer.draw_text("fin", (SCREEN_WIDTH/2) as i32, (SCREEN_HEIGHT/2) as i32, 24, Color::WHITE);
			}
		}

		if show_debug
		{
			// Render FPS 
			drawer.draw_fps(0, 0);

			// Display mouse position
			let mouse_pos_text: String = format!("MOUSE POS: ({}, {})", mouse_pos.x, mouse_pos.y);

			drawer.draw_text(&mouse_pos_text, 0, 20, 20, Color::LIGHTGREEN);
		}

		// Watermark
		drawer.draw_text("mechinsam.com", SCREEN_WIDTH-210, SCREEN_HEIGHT-30, 30, Color::BLACK);
	}
}
