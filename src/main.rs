use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{Color, Sampler, Canvas};
use ggez::event::{self, EventHandler};
use ggez::glam::*;
use ggez::conf::*;
use ggez::input::keyboard::{KeyInput, KeyCode};
use std::collections::HashSet;

mod player;
use player::Player;

mod camera;
use camera::Camera;

mod map;
use map::Map;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("RPG Game", "SaltyCakeMix")
        .window_setup(WindowSetup::default().title("test test!").vsync(false))
        .window_mode(WindowMode::default().dimensions(800.0, 600.0).resizable(true))
        .backend(Backend::Dx12)
        .add_resource_path(std::path::PathBuf::from("./assets"))
        .build()
        .expect("well that didn't work");

    let state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, state);
}

struct MainState {
    player: Player,
    keys_held: HashSet<KeyCode>,
    dt: f32,
    camera: Camera,
    map: Map,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        MainState {
            player: Player::new(ctx, "/sprites/Princess0001.png"),
            keys_held: HashSet::new(),
            dt: 0.0,
            camera: Camera::new(800.0, 600.0, 2.0),
            map: Map::new(ctx, "/levels/tiles.png", 20, 20),
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta().as_nanos() as f32 / 1_000_000_000.0; // To seconds
        
        // Move player and camera
        let dx: i32 = self.keys_held.contains(&KeyCode::D) as i32 - self.keys_held.contains(&KeyCode::A) as i32;
        let dy: i32 = self.keys_held.contains(&KeyCode::S) as i32 - self.keys_held.contains(&KeyCode::W) as i32;
        let mut tdx = dx as f32 * self.dt * self.player.speed; // Player speed used here since it's also used for camera movement
        let mut tdy = dy as f32 * self.dt * self.player.speed;
        if dx != 0 && dy != 0 {
            const ROOT2: f32 = 1.41421356237;
            tdx /= ROOT2;
            tdy /= ROOT2;
        }
        self.player.translate(tdx, tdy);
        self.camera.translate(tdx, tdy);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen
        let mut canvas = Canvas::from_frame(ctx, Color::from((0, 0, 255, 0)));
        canvas.set_sampler(Sampler::nearest_clamp());
        
        // Draw the map
        if !self.map.is_rendered() {
            self.map.render(ctx);
        }
        self.map.draw(&mut canvas, &mut self.camera);

        // Draw the player
        self.player.draw(&mut canvas, &mut self.camera);

        canvas.finish(ctx)
    }

    // Key checking
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if let Some(key) = input.keycode {
            self.keys_held.insert(key);
        }
        Ok(())
    }
    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult {
        if let Some(key) = input.keycode {
            self.keys_held.remove(&key);
        }
        Ok(())
    }

    // Centers camera whenever the window is resized
    fn resize_event(&mut self, _ctx: &mut Context, width: f32, height: f32) -> GameResult {
        self.camera.resize(width, height);
        Ok(())
    }
}