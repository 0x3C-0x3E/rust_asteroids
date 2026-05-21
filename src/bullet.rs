use macroquad::prelude::*;

use crate::PI;
use crate::enemy::Enemy;
use crate::player::Wrappable;
use crate::{SCALE, game::Entity};

#[derive(Debug, Default)]
pub struct Bullet {
    pos: Vec2,
    vel: f32,

    rot: f32,
    animation_tick: f32,
    animation_frame: f32,
}

fn squared_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)
}

impl Bullet {
    pub fn new(x: f32, y: f32, rot: f32) -> Self {
        Self {
            pos: Vec2 { x, y },
            vel: 600.0,
            rot: rot,
            ..Default::default()
        }
    }

    pub fn get_enemy_collision(&self, enemies: &[Enemy]) -> Option<usize> {
        enemies
            .iter()
            .position(|enemy| squared_distance(enemy.get_pos(), self.get_pos()) < 40.0_f32.powi(2))
    }
}

impl Entity for Bullet {
    fn draw(&self, texture: &Texture2D) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(16.0 * SCALE, 16.0 * SCALE)),
            source: Some(Rect::new(16.0 * self.animation_frame, 0.0, 16.0, 16.0)),
            rotation: self.rot + PI * 0.5,
            ..Default::default()
        };
        draw_texture_ex(texture, self.pos.x, self.pos.y, WHITE, params);
    }

    fn update(&mut self) {
        self.animation_tick += 1.0 * get_frame_time();
        if self.animation_tick >= 0.15 {
            self.animation_tick = 0.0;
            self.animation_frame = (self.animation_frame + 1.0) % 2.0;
        }
        self.pos.x += f32::cos(self.rot) * self.vel * get_frame_time();
        self.pos.y += f32::sin(self.rot) * self.vel * get_frame_time();
        self.wrap();
    }

    fn get_pos(&self) -> (f32, f32) {
        (self.pos.x, self.pos.y)
    }
}

impl Wrappable for Bullet {
    fn constraints(&mut self) -> (&mut Vec2, f32) {
        (&mut self.pos, 16.0)
    }
}
