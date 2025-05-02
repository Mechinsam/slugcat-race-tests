use raylib::prelude::*;
use std::{collections::HashMap, fs};
use serde_json::Value;

use crate::entity::Food;
use crate::texture_to_collision_mask;
use crate::rendersystem::Viewport;

pub struct Map
{
	pub map_name: String,
	pub background: Texture2D,
	pub col_map: Vec<bool>,
	pub food_spawn_pos: Vector2,
	pub gate_spawn_pos: Vector2,

	pub food: Food
}

impl Map
{
	// Probably not very efficent to pass in the viewport but,,,, joe well!
	pub fn new(_map_name: &str, viewport: &mut Viewport) -> Map
	{
		let map_name: String = _map_name.to_string();

		let bg_location: String = format!("DATA/maps/{}/bg.png", map_name);
		let colmap_location: String = format!("DATA/maps/{}/col_map.png", map_name);
		let food_texture_location: String = format!("DATA/maps/{}/food.png", map_name);
		let metadata_location: String = format!("DATA/maps/{}/metadata.json", map_name);


		let background: Texture2D = viewport.load_image(&bg_location);
		
		let col_map_texture: Texture2D = viewport.load_image(&colmap_location);
		let col_map: Vec<bool> = texture_to_collision_mask(&col_map_texture, 1f32);


		// JSON Parsing
		let metadata_file: String = fs::read_to_string(&metadata_location).expect("Failed to read map metadata");

		let metadata: HashMap<String, Value> = serde_json::from_str(&metadata_file).expect("Failed to parse map metadata");

		// i couldnt think of a simplier way :p
		let food_spawn_pos: Vector2 = metadata
			.get("food_spawn_pos")
			.and_then(Value::as_array)             // <— parse as array
			.and_then(|arr| {
				if arr.len() >= 2 {
				// extract x, y from arr[0], arr[1]
				let x = arr[0].as_f64().unwrap_or(0.0) as f32;
				let y = arr[1].as_f64().unwrap_or(0.0) as f32;
				Some(Vector2::new(x, y))
			} else {
				None
			}
		}).unwrap_or_else(|| Vector2::new(0.0, 0.0));
		
		println!("{}", food_spawn_pos.x);

		// THIS SHIT DOESNT WORK AND NEEDS CHANGING EVENTUALLY!!!!!!!
		let gate_spawn_pos: Vector2 = metadata
			.get("gate_spawn_pos")
			.and_then(Value::as_array)             // <— parse as array
			.and_then(|arr| {
				if arr.len() >= 2 {
				// extract x, y from arr[0], arr[1]
				let x = arr[0].as_f64().unwrap_or(0.0) as f32;
				let y = arr[1].as_f64().unwrap_or(0.0) as f32;
				Some(Vector2::new(x, y))
			} else {
				None
			}
		}).unwrap_or_else(|| Vector2::new(0.0, 0.0));
		
		// Food
		let food: Food = Food::new(viewport.load_image(&food_texture_location), 0.08, food_spawn_pos);

		Map {
			map_name,
			background,
			col_map,
			food_spawn_pos,
			gate_spawn_pos,
			food
		}
	}
}
