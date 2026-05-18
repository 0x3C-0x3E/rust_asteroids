use crate::PI;
use crate::SCALE;
use crate::game::Entity;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Player {
    pos: Vec2,
    vel: f32,
    acc: f32,

    rot: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: 0.0,
            acc: 0.0,
            rot: 0.0,
        }
    }
}

impl Entity for Player {
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
        if is_key_down(KeyCode::W) {
            self.acc = 5000.0;
        } else if is_key_down(KeyCode::S) {
            self.acc = -5000.0;
        } else {
            self.acc = 0.0;
        }

        if is_key_down(KeyCode::A) {
            self.rot -= 5.0 * get_frame_time();
        } else if is_key_down(KeyCode::D) {
            self.rot += 5.0 * get_frame_time();
        }

        self.vel += self.acc * get_frame_time() * 0.8;
        self.vel = self.vel.clamp(-800.0, 800.0);

        self.pos.x += f32::cos(self.rot) * self.vel * get_frame_time();
        self.pos.y += f32::sin(self.rot) * self.vel * get_frame_time();

        self.wrap();
    }
}

pub trait Wrappable {
    fn constraints(&mut self) -> (&mut Vec2, f32);

    fn wrap(&mut self) {
        let (pos, size) = self.constraints();

        if pos.x + size * SCALE < 0.0 {
            pos.x = screen_width();
        }

        if pos.x - size * SCALE > screen_width() {
            pos.x = 0.0;
        }

        if pos.y + size * SCALE < 0.0 {
            pos.y = screen_height();
        }

        if pos.y - size * SCALE > screen_height() {
            pos.y = 0.0;
        }
    }
}

impl Wrappable for Player {
    fn constraints(&mut self) -> (&mut Vec2, f32) {
        (&mut self.pos, 16.0)
    }
}
