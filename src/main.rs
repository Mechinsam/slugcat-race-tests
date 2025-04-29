use raylib::prelude::*;

mod entity;
mod physics;
mod rendersystem;

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const MAXFPS: u32 = 75;
const DRAWFPS: bool = true;

fn main() {
	let mut viewport: rendersystem::Viewport = rendersystem::Viewport::init(
		SCREEN_WIDTH,
		SCREEN_HEIGHT,
		MAXFPS
	);
	
	let mut specimen: entity::Entity = entity::Entity::new(viewport.load_image("DATA/slugcat1.png"));

	while !viewport.window.window_should_close() {
		let mut drawer = viewport.window.begin_drawing(&viewport.thread);

		specimen.update(SCREEN_WIDTH, SCREEN_HEIGHT);

		//drawer.draw_text("I LOVE SKIBIDI TOILET.", 24, 24, 48, Color::MAROON);
		drawer.clear_background(Color::BLACK);
		specimen.draw(&mut drawer);
		
		//drawer.draw_texture(&spec, 100, 100, Color::WHITE);

		if DRAWFPS {
			drawer.draw_fps(0, 0);
		}
	}
}
