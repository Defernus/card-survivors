use game::*;
use macroquad::prelude::*;

#[macroquad::main("Card Survivors")]
async fn main() {
    let assets = GameAssets::load().await;
    let mut game = GameState::default();

    loop {
        clear_background(WHITE);

        game.draw_state(&assets);

        next_frame().await
    }
}
