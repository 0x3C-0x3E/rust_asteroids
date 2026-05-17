use macroquad::{miniquad::window::set_window_position, prelude::*};

#[derive(Debug, Default)]
struct Player {
    x: i32,
    y: i32,

    x_vel: f32,
    y_vel: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }

    fn draw(&self) {
        draw_circle(self.x as f32, self.y as f32, 10.0, RED);
    }

    fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.y_vel = -5.0;
        }
    }
}

struct GameState {
    player: Player,
}

impl GameState {
    fn new() -> Self {
        Self {
            player: Player::new(
                (screen_width() / 2.0) as i32,
                (screen_height() / 2.0) as i32,
            ),
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
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let game = GameState::new();

    set_window_position(1920 / 2 - 200, 1080 / 2 - 200);

    loop {
        clear_background(BLACK);

        game.draw();
        next_frame().await
    }
}
