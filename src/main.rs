use macroquad::prelude::*;

struct Player {
    x: i32,
    y: i32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn update() {}
}

struct GameState {
    player: Player,
}

impl GameState {
    fn new(player: Player) -> Self {
        Self { player }
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
