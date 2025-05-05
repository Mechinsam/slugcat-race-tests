// Misc. helper functions

use rand::seq::IndexedRandom;
use raylib::prelude::*;
use std::{fs, path, io::Read};
use crate::entity;
use crate::enums::GameState;
use crate::Viewport;
pub struct iVector2
{
	pub x: i32,
	pub y: i32
}

impl iVector2
{
	pub fn new(x: i32, y: i32) -> Self
	{
		iVector2 {
			x,
			y
		}
	}
}

pub fn load_slugcats(viewport: &mut Viewport, gate_spawn_pos: Vector2) -> Vec<entity::Slugcat>
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
				.and_then(|ext: &std::ffi::OsStr| ext.to_str())
				.filter(|&ext| ext.eq_ignore_ascii_case("png"))
				.and_then(|_| path.file_name()                                           
					.and_then(|os: &std::ffi::OsStr| os.to_str())
					.map(|s: &str| s.to_owned()))
		})
		.collect();

	let mut slugcats: Vec<entity::Slugcat> = Vec::new();

	let mut counter: i32 = 0;
	let slugcats_spacing: i32 = 10; // Distance between slugcats. Kind of like... padding

	for slugcat_path in slugcat_textures
	{
		let slugcat_name: String = slugcat_path
			.strip_suffix(".png")
			.unwrap_or(&slugcat_path)
			.to_string();
		println!("{}", slugcat_name);
		
		let slugcat_texture_path: String = format!("DATA/racers/sprites/{}", slugcat_path);

		let mut slugcat: entity::Slugcat = entity::Slugcat::new(&slugcat_name,
			viewport.load_image(&slugcat_texture_path),
			0.25,
			gate_spawn_pos);
		let width: i32 = slugcat.texture.width() + slugcats_spacing;

		for _ in 0..counter {
			slugcat.position.x += width as f32 * slugcat.scale;
		}
		

		slugcats.push(slugcat);

		counter += 1;
	}
	
	return slugcats;
}

pub fn get_music_name(state: GameState) -> String
{
	let mut rng = rand::rng();
	let mut dir: &str = "DATA/music/race"; // default is to pick from a race track

	if state == GameState::Win {
		dir = "DATA/music/win";
	}

	let entries: fs::ReadDir = fs::read_dir(dir).expect(&format!("Failed to read {}", dir));

	// ignore everything that isnt a png
	let race_track_names: Vec<String> = entries
		.filter_map(Result::ok)
		.filter_map(|entry| {
			let path = entry.path();
			// If extension == "png", extract file_name as String; else skip
			path.extension()
				.and_then(|ext: &std::ffi::OsStr| ext.to_str())
				.filter(|&ext| ext.eq_ignore_ascii_case("mp3"))
				.and_then(|_| path.file_name()                                           
					.and_then(|os: &std::ffi::OsStr| os.to_str())
					.map(|s: &str| s.to_owned()))
		})
		.collect();
	
	let track_name: String = race_track_names.choose(&mut rng).unwrap().to_string();
	let track_path: String = format!("{dir}/{}", track_name);
	
	return  track_path;
}

pub fn get_map_name_from_file() -> String
{
	let mut file = fs::File::open("map.txt").expect("Could not open map.txt!");
	let mut map_name: String = String::new();

	file.read_to_string(&mut map_name).expect("Couldn't read 'map.txt'!");

	let map_path = format!("DATA/maps/{}", map_name);

	if !path::Path::new(&map_path).exists() {
		panic!("'{}': map does not exist!", map_name);
	}

	return map_name;
}
