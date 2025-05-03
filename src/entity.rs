use raylib::prelude::*;
use std::ops::{Deref, DerefMut};
use rand::Rng;

use crate::texture_to_collision_mask;

const MIN_SPEED: i32 = 050;
const MAX_SPEED: i32 = 150;

// "Super-class"
pub struct Entity
{
	pub position: Vector2,
	pub speed: Vector2,

	pub texture: Texture2D,

	width: i32,
	height: i32,
	mask: Vec<bool>,
	pub scale: f32
}

// Slugcat "Sub-class"
pub struct Slugcat
{
	pub entity: Entity,
	pub name: String,
}

impl Deref for Slugcat {
	type Target = Entity;
	fn deref(&self) -> &Entity {
		&self.entity
	}
}

impl DerefMut for Slugcat {
	fn deref_mut(&mut self) -> &mut Entity {
		&mut self.entity
	}
}

// Food "Sub-class"
pub struct Food
{
	pub entity: Entity
}

impl Deref for Food {
	type Target = Entity;
	fn deref(&self) -> &Entity {
		&self.entity
	}
}

impl DerefMut for Food {
	fn deref_mut(&mut self) -> &mut Entity {
		&mut self.entity
	}
}

#[derive(Clone)]
pub struct CollisionData {
	position: Vector2,
	width: i32,
	height: i32,
	mask: Vec<bool>,
	name: String,           // or an ID field if you prefer
}

impl Entity
{
	// PLEAAAAASE for the love of god make sure your 'scale' value for your entity WILL MEAN WIDTH AND HEIGHT WILL BE A WHOLE NUMBER
	pub fn new(texture: Texture2D, scale: f32) -> Self
	{
		let mask: Vec<bool> = texture_to_collision_mask(&texture, scale);

		let width: i32 = (texture.width() as f32 * scale) as i32;
		let height: i32 = (texture.height() as f32 * scale) as i32;

		Entity {
			position: Vector2::new(300f32, 200f32),
			speed: Vector2::new(MIN_SPEED as f32, MIN_SPEED as f32),
			texture,
			width,
			height,
			mask,
			scale
		}
	}

	pub fn to_collision_data(&self, name: &str) -> CollisionData {
		CollisionData {
			position: self.position,
			width:    self.width,
			height:   self.height,
			mask:     self.mask.clone(),
			name:     name.to_string(),
		}
	}

	pub fn draw(&self, drawer: &mut RaylibDrawHandle)
	{
		drawer.draw_texture_ex(&self.texture, self.position, 0f32, self.scale,Color::WHITE);
	}
}

impl Slugcat
{
	pub fn new(name: &str, texture: Texture2D, scale: f32, position: Vector2) -> Self
	{
		let mut entity: Entity = Entity::new(texture, scale);
		entity.position = position;

		let mut rng = rand::rng();
		entity.speed = Vector2::new(
			rng.random_range(MIN_SPEED..MAX_SPEED) as f32,
			rng.random_range(MIN_SPEED..MAX_SPEED) as f32
		);

		// randomly flip direction
		if rng.random_bool(0.5) {
			entity.speed.x = -entity.speed.x;
		}
		if rng.random_bool(0.5) {
			entity.speed.y = -entity.speed.y;
		}

		Slugcat {
			entity,
			name: name.to_string(),
		}
	}

	// I wouldn't try to touch this if i were you..... i have no clue HOW this works.... but it works
	pub fn update(
		&mut self,
		screen_width: i32,
		screen_height: i32,
		delta_time: f32,
		map_mask: &[bool],
		mask_width: i32,
		mask_height: i32,
		other_slugcats: &Vec<CollisionData>,
	) {
		let mut rng: rand::prelude::ThreadRng = rand::rng();
		let next_x = self.position.x + self.speed.x * delta_time;
		let next_y = self.position.y + self.speed.y * delta_time;
	
		// ─── HORIZONTAL TEST ────────────────────────────────────────────────────────
		let mut collided_x = false;
		'horizontal: for y in 0..self.height {
			for x in 0..self.width {
				let idx = (y * self.width + x) as usize;
				if !self.mask[idx] { continue; }
	
				let wx = next_x + x as f32;
				let wy = self.position.y + y as f32;
	
				// 1) map collision
				if wx >= 0.0 && wy >= 0.0
				   && wx < mask_width  as f32
				   && wy < mask_height as f32
				   && map_mask[(wy as i32 * mask_width + wx as i32) as usize]
				{
					collided_x = true;
					break 'horizontal;
				}
	
				// 2) entity–entity collision
				for other in other_slugcats.iter() {
					if other.name == self.name { continue; }
	
					let rel_x = (wx - other.position.x).floor() as i32;
					let rel_y = (wy - other.position.y).floor() as i32;
					if rel_x < 0 || rel_y < 0 || rel_x >= other.width || rel_y >= other.height {
						continue;
					}
					let other_idx = (rel_y * other.width + rel_x) as usize;
					if other.mask[other_idx] {
						collided_x = true;
						break 'horizontal;
					}
				}
			}
		}
		if collided_x {
			self.speed.x = -self.speed.x.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
		} else {
			self.position.x = next_x;
		}
	
		// ─── VERTICAL TEST ──────────────────────────────────────────────────────────
		let mut collided_y = false;
		'vertical: for y in 0..self.height {
			for x in 0..self.width {
				let idx = (y * self.width + x) as usize;
				if !self.mask[idx] { continue; }
	
				let wx = self.position.x + x as f32;
				let wy = next_y + y as f32;
	
				// 1) map collision
				if wx >= 0.0 && wy >= 0.0
				   && wx < mask_width  as f32
				   && wy < mask_height as f32
				   && map_mask[(wy as i32 * mask_width + wx as i32) as usize]
				{
					collided_y = true;
					break 'vertical;
				}
	
				// 2) entity–entity collision
				for other in other_slugcats.iter() {
					if other.name == self.name { continue; }
	
					let rel_x = (wx - other.position.x).floor() as i32;
					let rel_y = (wy - other.position.y).floor() as i32;
					if rel_x < 0 || rel_y < 0 || rel_x >= other.width || rel_y >= other.height {
						continue;
					}
					let other_idx = (rel_y * other.width + rel_x) as usize;
					if other.mask[other_idx] {
						collided_y = true;
						break 'vertical;
					}
				}
			}
		}
		if collided_y {
			self.speed.y = -self.speed.y.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
		} else {
			self.position.y = next_y;
		}
	
		// ─── SCREEN BOUNDS ──────────────────────────────────────────────────────────
		if self.position.x < 0.0 || self.position.x + self.width as f32 > screen_width as f32 {
			self.speed.x = -self.speed.x;
		}
		if self.position.y < 0.0 || self.position.y + self.height as f32 > screen_height as f32 {
			self.speed.y = -self.speed.y;
		}
	}
}

impl Food
{
	pub fn new(texture: Texture2D, scale: f32, position: Vector2) -> Self
	{
		let mut entity: Entity = Entity::new(texture, scale);
		entity.position = position;

		Food {entity}
	}

	// Food does not use pixel perfect collisions but still initilises it when calling Food::new. [TODO] remove that somehow
	pub fn update(&self, slugcats: &Vec<Slugcat>) -> String
	{
		// Default value is '/' because the slugcat name is its filename so we dont want the default value to clash with potential filenames
		// But '/' is an illegal filename on a lot of operating systems hence why it was chosen.
		let default: String= String::from("/");

		let food_col_box: Rectangle = Rectangle::new(
			self.position.x,
			self.position.y,
			self.width as f32,
			self.height as f32,
		);

		// Create slugcat collision box and check for collisions
		for slugcat in slugcats
		{
			let slugcat_col_box: Rectangle = Rectangle::new(
				slugcat.position.x,
				slugcat.position.y,
				slugcat.width as f32,
				slugcat.height as f32
			);

			if slugcat_col_box.check_collision_recs(&food_col_box)
			{
				println!("yeah im eating this shit");
				return slugcat.name.clone();
			}
		}

		return default;
	}
}
