use std::cmp::Ordering;

use crate::*;
use macroquad::prelude::*;

pub const CARDS_TO_DISPLAY: usize = 5;

pub struct GameState {
    current_deck: CardDeck,
    drawn_cards: Vec<Card>,
    prev_card: Card,
    money: u32,
    bet: u32,
}

impl GameState {
    pub fn new() -> Self {
        let mut current_deck = CardDeck::default();

        let drawn_cards = vec![current_deck.next().unwrap()];

        Self {
            prev_card: drawn_cards[0],
            current_deck,
            drawn_cards,
            money: 100,
            bet: 10,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Cap the bet to the current money.
    fn normalize_bet(&mut self) {
        self.bet = self.bet.min(self.money);
    }

    pub fn update_state(&mut self) {
        if self.money == 0 {
            if is_key_pressed(KeyCode::Space) {
                self.reset();
            }

            return;
        }

        self.handle_update_bet();
        self.handle_next_card();
    }

    fn handle_update_bet(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.bet += 1;
        }

        if is_key_down(KeyCode::Left) {
            self.bet = self.bet.saturating_sub(1);
        }

        self.normalize_bet();
    }

    fn handle_next_card(&mut self) {
        let is_higher = is_key_pressed(KeyCode::Up);
        let is_lower = is_key_pressed(KeyCode::Down);

        if is_higher || is_lower {
            let card = self.next_card();
            let comparison = card.compare(&self.prev_card);

            self.normalize_bet();
            self.money = self.money.saturating_sub(self.bet);

            match (is_higher, comparison) {
                // draw again
                (_, Ordering::Equal) => {
                    self.money += self.bet;
                }
                // player wins
                (true, Ordering::Greater) | (false, Ordering::Less) => {
                    self.money += self.bet * 2;
                }
                // player loses
                (true, Ordering::Less) | (false, Ordering::Greater) => {} // card ranks are equal
            }

            self.bet = 0;
        }
    }

    fn next_card(&mut self) -> Card {
        if let Some(card) = self.current_deck.next() {
            self.drawn_cards.push(card);
            card
        } else {
            self.current_deck = CardDeck::default();
            let card = self.current_deck.next().unwrap();
            self.drawn_cards.push(card);
            card
        }
    }

    pub fn draw_state(&mut self, assets: &GameAssets) {
        self.draw_cards(assets);

        let text_params = TextParams {
            font: Some(&assets.font_regular),
            font_size: 30,
            color: BLACK,
            ..Default::default()
        };

        draw_text_ex(
            &format!("Money: {}", self.money),
            10.0,
            30.0,
            text_params.clone(),
        );
        draw_text_ex(&format!("Bet: {}", self.bet), 10.0, 70.0, text_params);

        if self.money == 0 {
            self.draw_game_over(assets);
        }
    }

    fn draw_cards(&self, assets: &GameAssets) {
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

    fn draw_game_over(&self, assets: &GameAssets) {
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 200),
        );

        let font = &assets.font_regular;

        let game_over_params = TextParams {
            font: font.into(),
            font_size: 40,
            color: RED,
            ..Default::default()
        };
        let game_over_text = "Game Over";
        let game_over_text_dimensions = measure_text(
            game_over_text,
            game_over_params.font,
            game_over_params.font_size,
            game_over_params.font_scale,
        );
        let game_over_x = screen_width() * 0.5 - game_over_text_dimensions.width * 0.5;
        let game_over_y = screen_height() * 0.5 - game_over_text_dimensions.height * 0.5;

        let restart_params = TextParams {
            font: font.into(),
            font_size: 20,
            color: BLACK,
            ..Default::default()
        };
        let restart_text = "Press Space to restart";
        let restart_text_dimensions = measure_text(
            restart_text,
            restart_params.font,
            restart_params.font_size,
            restart_params.font_scale,
        );
        let restart_x = screen_width() * 0.5 - restart_text_dimensions.width * 0.5;
        let restart_y = game_over_y + game_over_text_dimensions.height + 10.0;

        draw_text_ex("Game Over", game_over_x, game_over_y, game_over_params);
        draw_text_ex(
            "Press Space to restart",
            restart_x,
            restart_y,
            restart_params,
        );
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
