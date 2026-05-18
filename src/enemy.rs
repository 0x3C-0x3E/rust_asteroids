use crate::PI;
use crate::SCALE;
use crate::game::Entity;
use crate::player::Wrappable;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Enemy {
    pos: Vec2,
    vel: f32,

    texture: Texture2D,

    rot: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Self {
        Self {
            pos: Vec2::new(x, y),
            texture: texture,
            vel: 0.0,
            rot: 0.0,
        }
    }
}

impl Entity for Enemy {
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
        self.vel = 500.0;

        self.pos.x += f32::cos(self.rot) * self.vel * get_frame_time();
        self.pos.y += f32::sin(self.rot) * self.vel * get_frame_time();

        self.wrap();
    }
}

impl Wrappable for Enemy {
    fn constraints(&mut self) -> (&mut Vec2, f32) {
        (&mut self.pos, 16.0)
    }
}
