use crate::*;
use macroquad::window::screen_height;

pub const CARDS_TO_DISPLAY: usize = 5;

pub struct GameState {
    pub current_deck: CardDeck,
    pub drawn_cards: Vec<Card>,
}

impl GameState {
    pub fn new() -> Self {
        let mut current_deck = CardDeck::default();

        let drawn_cards = vec![current_deck.next().unwrap()];

        Self {
            current_deck,
            drawn_cards,
        }
    }

    pub fn update_state(&mut self) {}

    pub fn draw_state(&mut self, assets: &GameAssets) {
        for (index, card) in self
            .drawn_cards
            .iter()
            .rev()
            .take(CARDS_TO_DISPLAY)
            .enumerate()
            .rev()
        {
            let offset = Card::WIDTH * 0.5;

            let x = CARDS_TO_DISPLAY as f32 * offset - index as f32 * offset;
            let y = screen_height() * 0.5 - Card::HEIGHT * 0.5;

            card.draw(assets, x, y);
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
