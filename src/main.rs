use crate::game::GameState;
use std::f32::consts::PI;

use macroquad::{miniquad::window::set_window_position, prelude::*};

const SCALE: f32 = 3.0;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

mod bullet;
mod enemy;
mod game;
mod player;

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
