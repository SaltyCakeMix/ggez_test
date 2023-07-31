use ggez::graphics::{Image, Canvas, DrawParam, Color};
use ggez::glam::Vec2;
use rand::Rng;

use crate::Camera;

const TILE_SIZE: u32 = 16;

pub struct Map {
	sprite: Option<Image>,
	grid: Vec<Vec<u8>>,
	tile_set: Vec<Image>,
	tile_sheet: Image,
	w: u32,
	h: u32,
}

impl Map {
	pub fn new(ctx: &mut ggez::Context, path: &str, w: u32, h: u32) -> Map {
		let tile_set: Vec<Image> = Vec::new();
		let tile_sheet = Image::from_path(ctx, path).unwrap();
		let grid: Vec<Vec<u8>> = Vec::new();
		Map {sprite: None, grid: grid, tile_set: tile_set, tile_sheet: tile_sheet, w: w, h: h}
	}
	
	pub fn draw(&self, canvas: &mut Canvas, camera: &Camera) {
		if let Some(sprite) = &self.sprite {
			canvas.draw(sprite, DrawParam::new()
				.dest(Vec2::new(-camera.x, -camera.y))
				.scale(Vec2::new(camera.zoom, camera.zoom))
			);
		}
	}

	pub fn render(&mut self, ctx: &mut ggez::Context) {
		let color_format = ctx.gfx.surface_format();

		// Reads tile data
		if self.tile_set.len() == 0 {
			for x in (0..self.tile_sheet.width()).step_by(TILE_SIZE as usize) {
				for y in (0..self.tile_sheet.height()).step_by(TILE_SIZE as usize) {
					let tile = Image::new_canvas_image(ctx, color_format, TILE_SIZE, TILE_SIZE, 1);
					let mut canvas = Canvas::from_image(ctx, tile.clone(), Color::from((0, 0, 0, 0)));

					let draw_param = DrawParam::new().dest(Vec2::new(-(x as f32), -(y as f32)));
					canvas.draw(&self.tile_sheet, draw_param);

					let _ = canvas.finish(ctx);

					self.tile_set.push(tile);
				}
			}
		}

		// Create the render
		let render = Image::new_canvas_image(ctx, color_format, self.w*TILE_SIZE, self.h*TILE_SIZE, 1);
		let mut canvas = Canvas::from_image(ctx, render.clone(), Color::from((0, 0, 0, 0)));

		let mut rng = rand::thread_rng();
		for i in 0..self.w {
			let mut row: Vec<u8> = Vec::new();
			for k in 0..self.h {
				let n = rng.gen_range(0..30);
				row.push(n);
				canvas.draw(&self.tile_set[n as usize], DrawParam::new()
					.dest(Vec2::new((i*TILE_SIZE) as f32, (k*TILE_SIZE) as f32)));
			}
			self.grid.push(row);
		}

		let _ = canvas.finish(ctx);

		self.sprite = Some(render);
	}

	pub fn is_rendered(&self) -> bool {
		return self.sprite.is_some();
	}
}