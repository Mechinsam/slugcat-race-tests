// This file contains the info for loading slugcat sprites and win screens
use raylib::prelude::*;
use std::{path::Path, fs};

use crate::entity;
use crate::Viewport;

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

	for slugcat_texture_path in slugcat_textures
	{
		let slugcat_name: String = slugcat_texture_path
			.strip_suffix(".png")
			.unwrap_or(&slugcat_texture_path)
			.to_string();
		println!("{}", slugcat_name);
		
		let slugcat_texture_path: String = format!("DATA/racers/sprites/{}", slugcat_texture_path);

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
