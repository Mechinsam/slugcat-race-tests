use raylib::prelude::*;

// ima be fr idk how the fuck this works but it calculates collisions
fn mask_solid(position: Vector2, mask: &[bool], mask_width: i32, mask_height: i32) -> bool
{
	if position.x < 0f32 || position.y < 0f32 || position.x >= mask_width as f32 || position.y >= mask_height as f32
	{
		return false;
	}

	return mask[(position.y.floor() as i32 * mask_width + position.x.floor() as i32) as usize];
}

pub struct Entity
{
	pub position: Vector2,
	pub speed: Vector2,

	pub texture: Texture2D,

	width: i32,
	height: i32
}

impl Entity
{
	pub fn new(texture: Texture2D) -> Self
	{
		let width: i32 = texture.width();
		let height: i32 = texture.height();

		Entity {
			position: Vector2::new(200f32, 200f32),
			speed: Vector2::new(200f32, 200f32),
			texture,
			width,
			height
		}
	}

	// I wouldn't try to touch this if i were you..... i have no clue what half of this function does but it works
	pub fn update(&mut self, screen_width: i32, screen_height: i32, delta_time: f32, mask: &[bool], mask_width: i32, mask_height: i32)
	{
		let mut dest: Vector2 = Vector2::new(0f32, 0f32);
		dest.x = self.position.x + self.speed.x * delta_time;
		dest.y = self.position.y + self.speed.y * delta_time;
	
		// x-axis collision check
		if self.speed.x != 0.0 {
			let edge_x = if self.speed.x > 0.0 {
				dest.x + self.width as f32 - 1.0   // right edge
		} else {
			dest.x
		};
		let mut hit_x = false;
		for y_off in 0..(self.height as f32 as i32) {
			let sample = Vector2::new(edge_x, self.position.y + y_off as f32);
			if mask_solid(sample, mask, mask_width, mask_height) {
				hit_x = true;
				break;
		 }
		}
		if hit_x {
		 self.speed.x = -self.speed.x;  // bounce on X
		} else {
		 self.position.x = dest.x;      // commit X
		}
		}
	
		// 3. Vertical axis: choose leading edge and sample width
		if self.speed.y != 0.0 {
		 let edge_y = if self.speed.y > 0.0 {
			 dest.y + self.height as f32 - 1.0   // bottom edge
		 } else {
			 dest.y            // top edge
		 };
		 let mut hit_y = false;
		 for x_off in 0..(self.width as f32 as i32) {
			 let sample = Vector2::new(self.position.x + x_off as f32, edge_y);
			 if mask_solid(sample, mask, mask_width, mask_height) {
				 hit_y = true;
				 break;
			 }
		 }
		 if hit_y {
			 self.speed.y = -self.speed.y;
		 } else {
			 self.position.y = dest.y;
		 }
		}

		// screen collisions
		if self.position.x < 0f32 || self.position.x + self.width as f32 > screen_width as f32
		{
			self.speed.x = -self.speed.x
		}
		if self.position.x < 0f32 || self.position.x + self.width as f32 > screen_width as f32
		{
			self.speed.x = -self.speed.x
		}
	}

	pub fn draw(&self, drawer: &mut RaylibDrawHandle)
	{
		drawer.draw_texture(&self.texture, self.position.x as i32, self.position.y as i32, Color::WHITE);
	}
}
