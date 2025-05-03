// Self-explanitory by filename

pub struct Timer {
	remaining: f32, // seconds remaining
	max: f32 // Keeps track of the original time the timer was set for
}

impl Timer
{
	pub fn new() -> Self
	{
		Timer {
			remaining: 0f32,
			max: 0f32
		}
	}
	
	pub fn set(&mut self, seconds: f32)
	{
		self.remaining = seconds;
		self.max = seconds;
	}

	pub fn get_max(&self) -> f32
	{
		return self.max;
	}

	// should be called every frame
	pub fn tick(&mut self, delta_time: f32)
	{
		if self.remaining > 0f32 {
			self.remaining -= delta_time;
		}
	}

	// returns true if timer is less than 0
	pub fn is_done(&self) -> bool
	{
		self.remaining <= 0.0
	}

	// returns how many seconds are remaining
	pub fn seconds_remaining(&self) -> f32
	{
		self.remaining.max(0f32)
	}
}
