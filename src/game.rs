use std::f32::consts::FRAC_PI_2;

use crate::{SCALE, player::Player};
use macroquad::prelude::*;

pub struct GameState {
    player: Player,
    bg_texture: Texture2D,
}

impl GameState {
    pub async fn new() -> Self {
        let player_texture = load_texture("assets/player.png").await.unwrap();
        let bg_texture = load_texture("assets/bg.png").await.unwrap();
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
        let tiles_x: u32 = (screen_width() / (64.0 * SCALE) + 1.0) as u32;
        let tiles_y: u32 = (screen_height() / (64.0 * SCALE) + 1.0) as u32;
        for y in 0..tiles_y {
            for x in 0..tiles_x {
                let params = DrawTextureParams {
                    dest_size: Some(vec2(64.0 * SCALE, 64.0 * SCALE)),
                    source: Some(Rect::new(0.0, 0.0, 64.0, 64.0)),
                    rotation: FRAC_PI_2 * ((x + y) % 4) as f32,
                    ..Default::default()
                };
                draw_texture_ex(
                    &self.bg_texture,
                    x as f32 * 64.0 * SCALE,
                    y as f32 * 64.0 * SCALE,
                    WHITE,
                    params,
                );
            }
        }
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
