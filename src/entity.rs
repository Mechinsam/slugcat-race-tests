use raylib::prelude::*;
//use rand::Rng;

use crate::texture_to_collision_mask;

// ima be fr idk how the fuck this works but it calculates collisions
/*fn mask_solid(position: Vector2, mask: &[bool], mask_width: i32, mask_height: i32) -> bool
{
	if position.x < 0f32 || position.y < 0f32 || position.x >= mask_width as f32 || position.y >= mask_height as f32
	{
		return false;
	}

	return mask[(position.y.floor() as i32 * mask_width + position.x.floor() as i32) as usize];
}*/

pub struct Entity
{
	pub position: Vector2,
	pub speed: Vector2,

	pub texture: Texture2D,

	width: i32,
	height: i32,
	mask: Vec<bool>,
	scale: f32
}

impl Entity
{
	// PLEAAAAASE for the love of god make sure your 'scale' value for your entity WILL MEAN WIDTH AND HEIGHT WILL BE A WHOLE NUMBER
	pub fn new(texture: Texture2D, scale: f32) -> Self
	{
		let mask: Vec<bool> = texture_to_collision_mask(&texture, 0.5);

		let width: i32 = (texture.width() as f32 * scale) as i32;
		let height: i32 = (texture.height() as f32 * scale) as i32;

		Entity {
			position: Vector2::new(200f32, 200f32),
			speed: Vector2::new(200f32, 200f32),
			texture,
			width,
			height,
			mask,
			scale
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
	) {
		// compute proposed positions
		let next_x = self.position.x + self.speed.x * delta_time;
		let next_y = self.position.y + self.speed.y * delta_time;
	
		// ─── HORIZONTAL TEST ────────────────────────────────────────────────────────
		let mut collided_x = false;
		'horizontal: for y in 0..self.height {
			for x in 0..self.width {
				let idx = (y * self.width + x) as usize;
				if !self.mask[idx] { continue; }                // skip transparent pixels
				let wx = next_x + x as f32;
				let wy = self.position.y + y as f32;            // note: OLD y
				if wx >= 0.0 && wy >= 0.0
				   && wx < mask_width  as f32
				   && wy < mask_height as f32
				   && map_mask[(wy as i32 * mask_width + wx as i32) as usize]
				{
					collided_x = true;
					break 'horizontal;
				}
			}
		}
		if collided_x {
			self.speed.x = -self.speed.x;                      // reverse only X
		} else {
			self.position.x = next_x;                          // commit only X
		}
	
		// ─── VERTICAL TEST ──────────────────────────────────────────────────────────
		let mut collided_y = false;
		'vertical: for y in 0..self.height {
			for x in 0..self.width {
				let idx = (y * self.width + x) as usize;
				if !self.mask[idx] { continue; }
				let wx = self.position.x + x as f32;            // note: UPDATED x
				let wy = next_y + y as f32;
				if wx >= 0.0 && wy >= 0.0
				   && wx < mask_width  as f32
				   && wy < mask_height as f32
				   && map_mask[(wy as i32 * mask_width + wx as i32) as usize]
				{
					collided_y = true;
					break 'vertical;
				}
			}
		}
		if collided_y {
			self.speed.y = -self.speed.y;                      // reverse only Y
		} else {
			self.position.y = next_y;                          // commit only Y
		}
	
		// ─── SCREEN BOUNDS (optional) ───────────────────────────────────────────────
		if self.position.x < 0.0 || self.position.x + self.width as f32 > screen_width as f32 {
			self.speed.x = -self.speed.x;
		}
		if self.position.y < 0.0 || self.position.y + self.height as f32 > screen_height as f32 {
			self.speed.y = -self.speed.y;
		}
	}

	pub fn draw(&self, drawer: &mut RaylibDrawHandle)
	{
		drawer.draw_texture_ex(&self.texture, self.position, 0f32, self.scale,Color::WHITE);
	}
}
