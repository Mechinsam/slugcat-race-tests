use raylib::prelude::*;
use std::{collections::HashMap, fs};
use serde_json::Value;

use crate::texture_to_collision_mask;
use crate::rendersystem::Viewport;

pub struct Map
{
	pub map_name: String,
	pub background: Texture2D,
	pub col_map: Vec<bool>,
	pub food_spawn_pos: Vector2,
	pub gate_spawn_pos: Vector2
}

impl Map
{
	// Probably not very efficent to pass in the viewport but,,,, joe well!
	pub fn new(_map_name: &str, viewport: &mut Viewport) -> Map
	{
		let map_name: String = _map_name.to_string();

		let bg_location: String = format!("DATA/maps/{}/bg.png", map_name);
		let colmap_location: String = format!("DATA/maps/{}/col_map.png", map_name);
		let metadata_location: String = format!("DATA/maps/{}/metadata.json", map_name);


		let background: Texture2D = viewport.load_image(&bg_location);
		
		let col_map_texture: Texture2D = viewport.load_image(&colmap_location);
		let col_map: Vec<bool> = texture_to_collision_mask(&col_map_texture, 1f32);


		// JSON Parsing
		let metadata_file: String = fs::read_to_string(&metadata_location).expect("Failed to read map metadata");

		let metadata: HashMap<String, Value> = serde_json::from_str(&metadata_file).expect("Failed to parse map metadata");
		println!("{:?}", metadata.get("food_spawn_pos"));

		// i couldnt think of a simplier way :p
		let food_spawn_pos: Vector2 = metadata.get("food_spawn_pos")
			.and_then(Value::as_object)
			.map_or(Vector2::new(0.0, 0.0), |obj: &serde_json::Map<String, Value>|
			{
				Vector2::new(
					obj.get("x").and_then(Value::as_f64).unwrap_or(0.0) as f32,
					obj.get("y").and_then(Value::as_f64).unwrap_or(0.0) as f32,
				)
			});

		let gate_spawn_pos: Vector2 = metadata.get("gate_spawn_pos")
			.and_then(Value::as_object)
			.map_or(Vector2::new(0.0, 0.0), |obj: &serde_json::Map<String, Value>|
			{
				Vector2::new(
					obj.get("x").and_then(Value::as_f64).unwrap_or(0.0) as f32,
					obj.get("y").and_then(Value::as_f64).unwrap_or(0.0) as f32,
				)
			});
		

		Map {
			map_name,
			background,
			col_map,
			food_spawn_pos,
			gate_spawn_pos
		}
	}
}
