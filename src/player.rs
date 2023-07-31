use ggez::graphics::{Image, Canvas, DrawParam};
use ggez::glam::Vec2;

use crate::camera::Camera;

pub struct Player {
	pub x: f32,
	pub y: f32,
	pub x_offset: f32,
	pub y_offset: f32,
	pub speed: f32,
	pub sprite: Image,
}

impl Player {
	pub fn new(ctx: &mut ggez::Context, path: &str) -> Player {
		let spr = Image::from_path(ctx, path).unwrap();
		Player {
			x: 0.0,
			y: 0.0,
			speed: 500.0,
			x_offset: spr.width() as f32 / -2.0,
			y_offset: spr.height() as f32 / -2.0,
			sprite: spr,
		}
	}
 
	pub fn translate(&mut self, dx: f32, dy: f32) {
		self.x += dx;
		self.y += dy;
	}

	pub fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
		canvas.draw(&self.sprite, DrawParam::new()
            .dest(Vec2::new(self.x - camera.x + self.x_offset*camera.zoom, self.y - camera.y + self.y_offset*camera.zoom))
            .scale(Vec2::new(camera.zoom, camera.zoom))
        );
	}
}