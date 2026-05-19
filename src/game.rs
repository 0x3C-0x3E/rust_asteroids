use std::f32::consts::FRAC_PI_2;

use crate::{SCALE, bullet::Bullet, enemy::Enemy, player::Player};
use macroquad::prelude::*;

pub struct Textures {
    player_texture: Texture2D,
    bg_texture: Texture2D,
    enemy_texture: Texture2D,
    bullet_texture: Texture2D,
}

impl Textures {
    pub async fn new() -> Self {
        Self {
            player_texture: load_texture("assets/player.png").await.unwrap(),
            bg_texture: load_texture("assets/bg.png").await.unwrap(),
            enemy_texture: load_texture("assets/enemy.png").await.unwrap(),
            bullet_texture: load_texture("assets/bullets.png").await.unwrap(),
        }
    }
}

pub struct GameState {
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,

    textures: Textures,
    spawn_counter: f32,
}

impl GameState {
    pub async fn new() -> Self {
        Self {
            player: Player::new(
                screen_width() / 2.0 - 16.0 * SCALE * 0.5,
                screen_height() / 2.0 - 16.0 * SCALE * 0.5,
            ),
            enemies: Vec::new(),
            bullets: Vec::new(),
            textures: Textures::new().await,
            spawn_counter: 0.0,
        }
    }

    pub fn spawn_enemy(&mut self) {
        self.enemies.push(Enemy::new(
            fastrand::f32() * screen_width(),
            fastrand::f32() * screen_height(),
        ));
    }

    fn spawn_bullet(&mut self) {
        self.bullets.push(Bullet::new(
            self.player.get_pos().0,
            self.player.get_pos().1,
            self.player.get_rot(),
        ));
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
                    &self.textures.bg_texture,
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
        for enemy in self.enemies.iter() {
            enemy.draw(&self.textures.enemy_texture);
        }

        for bullet in self.bullets.iter() {
            bullet.draw(&self.textures.bullet_texture);
        }

        self.player.draw(&self.textures.player_texture);
    }

    pub fn update(&mut self) {
        self.spawn_counter += 1.0 * get_frame_time();
        if self.spawn_counter >= 1.0 {
            self.spawn_counter = 0.0;
            self.spawn_enemy();
        }

        if self.player.should_shoot() {
            self.spawn_bullet();
        }

        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }

        for enemy in self.enemies.iter_mut() {
            enemy.update();
        }

        let mut hits: Vec<usize> = vec![];
        self.bullets.retain(|bullet| {
            if let Some(i) = bullet.get_enemy_collision(&self.enemies) {
                hits.push(i);
                true
            } else {
                false
            }
        });

        for hit in hits.iter().rev() {
            self.enemies.remove(*hit);
        }

        self.enemies.retain_mut(|e| e.tick());

        self.player.update();
    }
}

pub trait Entity {
    fn draw(&self, texture: &Texture2D);
    fn update(&mut self);

    fn get_pos(&self) -> (f32, f32);
}
