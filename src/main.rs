use macroquad::{miniquad::window::set_window_position, prelude::*};

#[derive(Debug)]
struct Player {
    pos: Vec2,
    vel: Vec2,

    texture: Texture2D,

    rot: f32,
}

impl Player {
    fn new(x: f32, y: f32, texture: Texture2D) -> Self {
        Self {
            pos: Vec2::new(x, y),
            texture: texture,
            vel: Vec2::ZERO,
            rot: 0.0,
        }
    }

    fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(16.0 * 4.0, 16.0 * 4.0)),
            source: Some(Rect::new(16.0, 0.0, 16.0, 16.0)),
            rotation: self.rot,
            ..Default::default()
        };
        draw_texture_ex(&self.texture, self.pos.x, self.pos.y, WHITE, params);
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.vel.y = -500.0;
        } else if is_key_down(KeyCode::S) {
            self.vel.y = 500.0;
        } else {
            self.vel.y = 0.0;
        }

        if is_key_down(KeyCode::A) {
            self.vel.x = -500.0;
        } else if is_key_down(KeyCode::D) {
            self.vel.x = 500.0;
        } else {
            self.vel.x = 0.0;
        }

        self.pos.x += self.vel.x * get_frame_time();
        self.pos.y += self.vel.y * get_frame_time();
    }
}

struct GameState {
    player: Player,
}

impl GameState {
    async fn new() -> Self {
        let texture = load_texture("assets/player.png").await.unwrap();
        Self {
            player: Player::new(screen_width() / 2.0, screen_height() / 2.0, texture),
        }
    }

    fn draw(&self) {
        self.player.draw();
    }

    fn update(&mut self) {
        self.player.update();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids!".to_owned(),
        window_width: 400,
        window_height: 400,
        window_resizable: false,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    set_window_position(1920 / 2 - 200, 1080 / 2 - 200);
    set_default_filter_mode(FilterMode::Nearest);
    let mut game = GameState::new().await;

    loop {
        game.update();

        clear_background(BLACK);

        game.draw();
        next_frame().await
    }
}
