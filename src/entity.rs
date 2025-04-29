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

	pub fn old_update(&mut self, screen_width: i32, screen_height: i32, delta_time: f32)
	{
		self.position.x += self.speed.x * delta_time;
		self.position.y += self.speed.y * delta_time;

		// Collision
		if self.position.x < 0.0 || self.position.x + self.texture.width() as f32 > screen_width as f32
		{
			self.speed.x = -self.speed.x;
		}
		if self.position.y < 0.0 || self.position.y + self.texture.height() as f32 > screen_height as f32
		{
			self.speed.y = -self.speed.y;
		}
	}

	pub fn update(&mut self, screen_width: i32, screen_height: i32, delta_time: f32, mask: &[bool], mask_width: i32, mask_height: i32)
	{
		// predict the next position
		let mut dest = Vector2::new(0f32, 0f32);
		dest.x = self.position.x + self.speed.x * delta_time;
		dest.y = self.position.y + self.speed.y * delta_time;


		// x-axis collision check
		let mut collided_x: bool = false;
		for y in 0..self.height
		{
			let check_pos = Vector2::new(dest.x, self.position.y + y as f32);
			if mask_solid(check_pos, mask, mask_width, mask_height)
			{
				collided_x = true;
				break;
			}
		}

		if collided_x
		{
			self.speed.x = -self.speed.x;
		} else {
			self.position.x = dest.x;
		}

		// y-axis collision check
		let mut collided_y: bool = false;
		for x in 0..self.width
		{
			let check_pos = Vector2::new(self.position.x + x as f32, dest.y);
			if mask_solid(check_pos, mask, mask_width, mask_height)
			{
				collided_y = true;
				break;
			}
		}

		if collided_y
		{
			self.speed.y = -self.speed.y;
		} else {
			self.position.y = dest.y;
		}

		// collision check
		/*if !mask_solid(dest, mask, mask_width, mask_height)
		{
			// no collision detected
			self.position = dest;
		} else {
			println!("ough");
			self.speed.x = -self.speed.x;
			self.speed.y = -self.speed.y;
		}*/
	}

	pub fn draw(&self, drawer: &mut RaylibDrawHandle)
	{
		drawer.draw_texture(&self.texture, self.position.x as i32, self.position.y as i32, Color::WHITE);
	}
}
