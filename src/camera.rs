pub struct Camera {
	pub x: f32,
	pub y: f32,
	pub zoom: f32,
	pub window_w: f32,
	pub window_h: f32,
}

impl Camera {
	pub fn new(window_w: f32, window_h: f32, zoom: f32) -> Camera {
		Camera {
			x: -window_w / 2.0,
			y: -window_h / 2.0,
			zoom: zoom,
			window_w: window_w,
			window_h: window_h,
		}
	}

	pub fn translate(&mut self, dx: f32, dy: f32) {
		self.x += dx;
		self.y += dy;
	}

	pub fn set_zoom(&mut self, new_zoom: f32) {
		self.zoom = new_zoom;
	}

	pub fn resize(&mut self, new_w: f32, new_h: f32) {
		self.translate((self.window_w - new_w) / 2.0, (self.window_h - new_h) / 2.0);
		self.window_w = new_w;
		self.window_h = new_h;
	}
}