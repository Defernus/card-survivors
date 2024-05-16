use game::*;
use macroquad::prelude::*;

#[macroquad::main("Card Survivors")]
async fn main() {
    let assets = GameAssets::load().await;
    let mut game = GameState::default();

    loop {
        clear_background(WHITE);

        game.draw_state(&assets);
        game.update_state();

        next_frame().await
    }
}
