// Despite being called physics, it does not handle physics. this is only here for the Vector2 struct.. lol..
pub struct Vector2
{
	pub x: f32,
	pub y: f32
}

impl Vector2
{
	pub fn new(_x: f32, _y:f32) -> Self
	{
		Vector2 {
			x: _x,
			y: _y
		}
	}
}
