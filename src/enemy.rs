use crate::PI;
use crate::SCALE;
use crate::game::Entity;
use crate::player::Wrappable;
use macroquad::prelude::*;

const LIFETIME: f32 = 4.0;

#[derive(Debug)]
pub struct Enemy {
    pos: Vec2,
    vel: f32,

    rot: f32,

    life_timer: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: fastrand::f32() * 80.0 + 50.0,
            rot: fastrand::f32() * PI,
            life_timer: 0.0,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.life_timer += 1.0 * get_frame_time();
        self.life_timer <= LIFETIME
    }
}

impl Entity for Enemy {
    fn draw(&self, texture: &Texture2D) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(16.0 * SCALE, 16.0 * SCALE)),
            source: Some(Rect::new(16.0, 0.0, 16.0, 16.0)),
            rotation: self.rot + PI * 0.5,
            ..Default::default()
        };
        draw_texture_ex(texture, self.pos.x, self.pos.y, WHITE, params);
    }

    fn update(&mut self) {
        self.pos.x += f32::cos(self.rot) * self.vel * get_frame_time();
        self.pos.y += f32::sin(self.rot) * self.vel * get_frame_time();

        self.wrap();
    }

    fn get_pos(&self) -> (f32, f32) {
        (self.pos.x, self.pos.y)
    }
}

impl Wrappable for Enemy {
    fn constraints(&mut self) -> (&mut Vec2, f32) {
        (&mut self.pos, 16.0)
    }
}
