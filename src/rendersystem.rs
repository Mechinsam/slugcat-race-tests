use raylib::prelude::*;

// Viewport is where the window and graphics library are initialised
pub struct Viewport
{
	pub window: RaylibHandle,
	pub thread: RaylibThread,
}

impl Viewport
{
	// Constructor
	pub fn init(title: &str, screen_width: i32, screen_height: i32, maxfps: u32) -> Self
	{
		let (mut window, thread) = raylib::init()
			.size(screen_width, screen_height)
			.title(title)
			//.vsync()
			.build();

		if maxfps > 300 {
			panic!("Max FPS must be equal to or less than 300");
		}

		// Set the target FPS
		window.set_target_fps(maxfps);

		// Return the initialized Viewport struct
		Viewport {
			window,
			thread
		}
	}

	pub fn load_image(&mut self, filename: &str) -> Texture2D
	{
		let image: Texture2D = self.window.load_texture(&self.thread, filename).expect("Failed to load texture");

		return image;
	}
}
