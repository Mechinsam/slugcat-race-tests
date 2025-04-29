use raylib::prelude::*;


pub struct Entity
{
	pub position: Vector2,
	pub speed: Vector2,

	pub texture: Texture2D
}

impl Entity
{
	pub fn new(texture: Texture2D) -> Self
	{
		Entity {
			position: Vector2::new(0f32, 0f32),
			speed: Vector2::new(2f32, 2f32),
			texture
		}
	}

	pub fn update(&mut self, screen_width: i32, screen_height: i32)
	{
		self.position.x += self.speed.x;
		self.position.y += self.speed.y;

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

	pub fn draw(&self, drawer: &mut RaylibDrawHandle)
	{
		drawer.draw_texture(&self.texture, self.position.x as i32, self.position.y as i32, Color::WHITE);
	}
}
