use std::f32::consts::PI;

use macroquad::{miniquad::window::set_window_position, prelude::*};

const SCALE: f32 = 3.0;

#[derive(Debug)]
struct Player {
    pos: Vec2,
    vel: f32,

    texture: Texture2D,

    rot: f32,
}

impl Player {
    fn new(x: f32, y: f32, texture: Texture2D) -> Self {
        Self {
            pos: Vec2::new(x, y),
            texture: texture,
            vel: 0.0,
            rot: 0.0,
        }
    }

    fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(16.0 * SCALE, 16.0 * SCALE)),
            source: Some(Rect::new(16.0, 0.0, 16.0, 16.0)),
            rotation: self.rot + PI * 0.5,
            ..Default::default()
        };
        draw_texture_ex(&self.texture, self.pos.x, self.pos.y, WHITE, params);
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.vel = 500.0;
        } else if is_key_down(KeyCode::S) {
            self.vel = -500.0;
        } else {
            self.vel = 0.0;
        }

        if is_key_down(KeyCode::A) {
            self.rot -= 5.0 * get_frame_time();
        } else if is_key_down(KeyCode::D) {
            self.rot += 5.0 * get_frame_time();
        }

        self.pos.x += f32::cos(self.rot) * self.vel * get_frame_time();
        self.pos.y += f32::sin(self.rot) * self.vel * get_frame_time();
    }
}

struct GameState {
    player: Player,
}

impl GameState {
    async fn new() -> Self {
        let texture = load_texture("assets/player.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        Self {
            player: Player::new(
                screen_width() / 2.0 - 16.0 * SCALE * 0.5,
                screen_height() / 2.0 - 16.0 * SCALE * 0.5,
                texture,
            ),
        }
    }

    fn draw_bg(&self) {}

    fn draw(&self) {
        self.draw_bg();
        self.player.draw();
    }

    fn update(&mut self) {
        self.player.update();
    }
}

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids!".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    set_window_position(1920 / 2 - WINDOW_WIDTH / 2, 1080 / 2 - WINDOW_HEIGHT / 2);
    set_default_filter_mode(FilterMode::Nearest);
    let mut game = GameState::new().await;

    loop {
        game.update();

        clear_background(BLACK);

        game.draw();
        next_frame().await
    }
}
