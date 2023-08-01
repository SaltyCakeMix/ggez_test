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
		let start = ctx.time.time_since_start().as_nanos();
		let color_format = ctx.gfx.surface_format();

		// Create the map
		let mut rng = rand::thread_rng();
		for _y in 0..self.h {
			let mut row: Vec<u8> = Vec::new();
			for _x in 0..self.w {
				let n = rng.gen_range(0..=1);
				row.push(n);
			}
			self.grid.push(row);
		}
		
		// Reads tile data
		if self.tile_set.len() == 0 {
			for y in (0..self.tile_sheet.height()).step_by(TILE_SIZE as usize) {
				for x in (0..self.tile_sheet.width()).step_by(TILE_SIZE as usize) {
					let tile = Image::new_canvas_image(ctx, color_format, TILE_SIZE, TILE_SIZE, 1);
					let mut canvas = Canvas::from_image(ctx, tile.clone(), Color::from((0, 0, 0, 0)));

					let draw_param = DrawParam::new().dest(Vec2::new(-(x as f32), -(y as f32)));
					canvas.draw(&self.tile_sheet, draw_param);

					let _ = canvas.finish(ctx);

					self.tile_set.push(tile);
				}
			}
		}

		// Draw tiles
		let render = Image::new_canvas_image(ctx, color_format, self.w*TILE_SIZE, self.h*TILE_SIZE, 1);
		let mut canvas = Canvas::from_image(ctx, render.clone(), Color::from((0, 0, 0, 0)));
		for y in 0..self.h {
			for x in 0..self.w {
				let arr = self.check_neighbors(x, y, self.w, self.h, 1);
				let tile_index: usize = match arr[..] {
					[_, false, _, false, true, true, _, true, true] 			=> 0,
					[_, false, _, true, true, true, true, true, true] 			=> 1,
					[_, false, _, true, true, false, true, true, _] 			=> 2,
					[_, false, _, false, true, true, _, false, _] 				=> 3,
					[_, false, _, true, true, true, _, false, _] 				=> 4,
					[_, false, _, true, true, false, _, false, _] 				=> 5,
					[_, true, true, false, true, true, _, true, true] 			=> 6,
					[true, true, true, true, true, true, true, true, true] 		=> 7,
					[true, true, _, true, true, false, true, true, _] 			=> 8,
					[_, false, _, false, true, false, _, false, _] 				=> 9,
					[_, false, _, false, true, false, _, true, _] 				=> 10,
					[_, _, _, _, false, _, _, _, _] 							=> 11,
					[_, true, true, false, true, true, _, false, _] 			=> 12,
					[true, true, true, true, true, true, _, false, _] 			=> 13,
					[true, 	true, _, true, true, false, _, false, _] 			=> 14,
					[false, true, false, true, true, true, false, true, false] 	=> 15,
					[_, true, _, false, true, false, _, true, _] 				=> 16,
					[false, true, true, true, true, true, _, true, _] 			=> 18,
					[true, true, false, true, true, true, _, true, _] 			=> 19,
					[_, false, _, false, true, true, _, true, false] 			=> 20,
					[_, false, _, true, true, false, false, true, _] 			=> 21,
					[_, true, _, false, true, false, _, false, _] 				=> 22,
					[_, true, _, true, true, true, false, true, true] 			=> 24,
					[_, true, _, true, true, true, true, true, false] 			=> 25,
					[_, true, false, false, true, true, _, false, _] 			=> 26,
					[false, true, _, true, true, false, _, false, _] 			=> 27,
					_ => 7,
				};

				canvas.draw(&self.tile_set[tile_index], DrawParam::new()
					.dest(Vec2::new((x*TILE_SIZE) as f32, (y*TILE_SIZE) as f32)));
			}
		}

		let _ = canvas.finish(ctx);

		self.sprite = Some(render);
		print!("{}\n", ctx.time.time_since_start().as_nanos() - start);
	}

	fn check_neighbors(&self, x: u32, y: u32, w: u32, h: u32, id: u8) -> Vec<bool> {
		let x: i32 = x as i32;
		let y: i32 = y as i32;
		let w: i32 = w as i32;
		let h: i32 = h as i32;
		let mut arr: Vec<bool> = Vec::new();
		for i in y-1..=y+1 {
			if i < 0 || i >= h {
				for _iter in 0..3 {
					arr.push(false);
				}
				continue;
			}
			for j in x-1..=x+1 {
				arr.push(j >= 0 && j < w && self.grid[i as usize][j as usize] == id);
			}
		}
		arr
	}

	pub fn is_rendered(&self) -> bool {
		return self.sprite.is_some();
	}
}