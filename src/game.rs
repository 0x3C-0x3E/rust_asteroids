use crate::{SCALE, player::Player};
use macroquad::prelude::*;

pub struct GameState {
    player: Player,
    bg_texture: Texture2D,
}

impl GameState {
    pub async fn new() -> Self {
        let player_texture = load_texture("assets/player.png").await.unwrap();
        let bg_texture = load_texture("assets/player.png").await.unwrap();
        Self {
            player: Player::new(
                screen_width() / 2.0 - 16.0 * SCALE * 0.5,
                screen_height() / 2.0 - 16.0 * SCALE * 0.5,
                player_texture,
            ),
            bg_texture: bg_texture,
        }
    }

    fn draw_bg(&self) {
        draw_texture(&self.bg_texture, 0.0, 0.0, WHITE);
    }

    pub fn draw(&self) {
        self.draw_bg();
        self.player.draw();
    }

    pub fn update(&mut self) {
        self.player.update();
    }
}

pub trait Entity {
    fn draw(&self);
    fn update(&mut self);
}
