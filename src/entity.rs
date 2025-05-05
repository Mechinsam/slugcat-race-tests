// Dude i aint gonna lie raylib is cool and all but im
// probably going to use godot for everything from now
// on coz HOLY SHIT this was a lot more work than i thought


use raylib::prelude::*;
use std::ops::{Deref, DerefMut};
use rand::Rng;

use crate::texture_to_collision_mask;
use crate::utils::iVector2;

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

	// this is a fucking mess btw
	// assumes mask width & height are the same as the screen size
	pub fn update (
		&mut self,
		screen_width: i32,
		screen_height: i32,
		delta_time: f32,
		map_mask: &[bool],
		other_slugcats: &Vec<CollisionData>
	) {
		// For determining random speeds
		let mut rng = rand::rng();

		// Calculate next potential position
		let dest: Vector2 = Vector2::new(
			self.position.x + self.speed.x * delta_time,
			self.position.y + self.speed.y * delta_time
		);
		
		// setting these for readability
		let mask_width: i32 = screen_width;
		let mask_height: i32 = screen_height;


		// horizontal collisions check
		// this is biiig and faat btw....
		let mut collided_x: bool = false;
		'horizontal_col_check: for y in 0..self.height
		{
			for x in 0..self.width
			{
				// usize used here to determine size of integer (64 on 64bit machines; 32 bit on 32bit machines)
				let index_x: usize = (y * self.width + x) as usize;

				// if current index is non collidable, skip this iteration
				if !self.mask[index_x] {continue;}

				let calculated_position: Vector2 = Vector2::new(
					dest.x + x as f32,
					self.position.y + y as f32
				);


				/* Map collision detection */
				if calculated_position.x >= 0f32
					&& calculated_position.y >= 0f32
					&& calculated_position.x < mask_width as f32 // If next possible x position is inside the map
					&& calculated_position.y < mask_height as f32 // And if next possible y position is inside the map
					&& map_mask[(calculated_position.y.floor() as i32 * mask_width + calculated_position.x.floor() as i32) as usize] // AND the position is collidable in the mask
				{
					// THEN we have collided with the map
					collided_x = true;
					break 'horizontal_col_check;
				}

				/* Entity on entity hate crime!!! (entity on entity collision) */
				for other_slugcat in other_slugcats.iter()
				{
					// Skip collision detection if the current slugcat is ourselves
					if other_slugcat.name == self.name {continue;}

					// Distance between slugcats
					let slug_distance: iVector2 = iVector2::new(
						((calculated_position.x - other_slugcat.position.x).floor() as i32),
						((calculated_position.y - other_slugcat.position.y).floor() as i32)
					);

					// Collision detection
					if slug_distance.x < 0 // If left horizontal distance is less than 0
						|| slug_distance.y < 0 // And if upper vertical distance is less than 0
						|| slug_distance.x >= other_slugcat.width // And if right horizontal distance is less than 0
						|| slug_distance.y >= other_slugcat.height // And if lower veritcal distance is less than 0
					{
						// No collision happened. Skip this iteration
						continue;
					}

					let other_slug_index: usize = (slug_distance.y * other_slugcat.width + slug_distance.x) as usize;
					
					// Check if we're overlapping into the other slugcat's mask
					if other_slugcat.mask[other_slug_index]
					{
						collided_x = true;
						break 'horizontal_col_check;
					}
				}
			}
		}

		// If we are collided horizontally, reverse and apply a random x speed
		if collided_x
		{
			// Signum returns 1 if value is positive and -1 if not
			self.speed.x = -self.speed.x.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
		} else
		{
			// No collision. Just set the x position as the next potential value
			self.position.x = dest.x;
		}

		///////////////////////
		

		// vertical collision check
		// also very biig and faat
		// theres probably a better way to do this wihtout copy pasting code but idk how
		let mut collided_y: bool = false;
		'veritcal_col_check: for y in 0..self.height
		{
			for x in 0..self.width
			{
				// usize used here to determine size of integer (64 on 64bit machines; 32 bit on 32bit machines)
				let index_x: usize = (y * self.width + x) as usize;

				// if current index is non collidable, skip this iteration
				if !self.mask[index_x] {continue;}

				let calculated_position: Vector2 = Vector2::new(
					self.position.x,
					dest.y + y as f32
				);


				/* Map collision detection */
				if calculated_position.x >= 0f32
					&& calculated_position.y >= 0f32
					&& calculated_position.x < mask_width as f32 // If next possible x position is inside the map
					&& calculated_position.y < mask_height as f32 // And if next possible y position is inside the map
					&& map_mask[(calculated_position.y.floor() as i32 * mask_width + calculated_position.x.floor() as i32) as usize] // AND the position is collidable in the mask
				{
					// THEN we have collided with the map
					collided_y = true;
					break 'veritcal_col_check;
				}

				/* Entity on entity hate crime!!! (entity on entity collision) */
				for other_slugcat in other_slugcats.iter()
				{
					// Skip collision detection if the current slugcat is ourselves
					if other_slugcat.name == self.name {continue;}

					// Distance between slugcats
					// Distance between slugcats
					let slug_distance: iVector2 = iVector2::new(
						((calculated_position.x - other_slugcat.position.x).floor() as i32),
						((calculated_position.y - other_slugcat.position.y).floor() as i32)
					);

					// Collision detection
					if slug_distance.x < 0 // If left horizontal distance is less than 0
						|| slug_distance.y < 0 // And if upper vertical distance is less than 0
						|| slug_distance.x >= other_slugcat.width // And if right horizontal distance is less than 0
						|| slug_distance.y >= other_slugcat.height // And if lower veritcal distance is less than 0
					{
						// No collision happened. Skip this iteration
						continue;
					}

					let other_slug_index: usize = (slug_distance.y * other_slugcat.width + slug_distance.x) as usize;
					
					// Check if we're overlapping into the other slugcat's mask
					if other_slugcat.mask[other_slug_index]
					{
						collided_y = true;
						break 'veritcal_col_check;
					}
				}
			}
		}

		// If we are collided vertically, reverse and apply a random y speed
		if collided_y
		{
			// Signum returns 1 if value is positive and -1 if not
			self.speed.y = -self.speed.y.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
		} else
		{
			// No collision. Just set the x position as the next potential value
			self.position.y = dest.y;
		}

		////////
		// Whew! Sorry about the repeating code im a bad programmer

		// Screen boundries
		if self.position.x < 0f32 || self.position.x + self.width as f32 > screen_width as f32
		{
			self.speed.x = -self.speed.x.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
		}
		if self.position.y < 0f32 || self.position.y + self.height as f32 > screen_height as f32
		{
			self.speed.y = -self.speed.y.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
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
				return slugcat.name.clone();
			}
		}

		return default;
	}
}
