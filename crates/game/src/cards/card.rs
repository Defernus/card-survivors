use crate::*;
use macroquad::prelude::*;
use std::cmp::Ordering;
use strum::EnumCount;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: CardRank,
    pub suit: CardSuit,
}

impl Card {
    pub const COUNT: usize = CardRank::COUNT * CardSuit::COUNT;
    pub const WIDTH: f32 = 100.0;
    pub const HEIGHT: f32 = 140.0;

    pub const ALL_VARIANTS: [Card; Self::COUNT] = get_all_variants();

    pub const fn new(rank: CardRank, suit: CardSuit) -> Self {
        Card { rank, suit }
    }

    pub fn compare(&self, other: &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }

    pub fn draw(self, assets: &GameAssets, x: f32, y: f32) {
        let color = self.suit.color();
        draw_rectangle_lines(x, y, Self::WIDTH, Self::HEIGHT, 10.0, color);

        self.suit.draw(assets, x, y);
        self.rank.draw(assets, x + 10.0, y + 10.0, color);
    }
}

const fn get_all_variants() -> [Card; Card::COUNT] {
    let mut cards = [Card {
        rank: CardRank::Two,
        suit: CardSuit::Hearts,
    }; Card::COUNT];
    let mut rank_index = 0;

    while rank_index < CardRank::COUNT {
        let mut suit_index = 0;

        while suit_index < CardSuit::COUNT {
            cards[rank_index * CardSuit::COUNT + suit_index] = Card {
                rank: CardRank::from_u8(rank_index as u8).unwrap(),
                suit: CardSuit::from_u8(suit_index as u8).unwrap(),
            };
            suit_index += 1;
        }
        rank_index += 1;
    }

    cards
}

#[test]
fn test_get_all_variants() {
    use strum::IntoEnumIterator;

    assert_eq!(Card::ALL_VARIANTS.len(), 52);
    assert_eq!(Card::COUNT, 52);

    let all_cards_expected = CardRank::iter()
        .flat_map(|rank| CardSuit::iter().map(move |suit| Card::new(rank, suit)))
        .collect::<Vec<_>>();

    assert_eq!(Card::ALL_VARIANTS.as_slice(), all_cards_expected.as_slice());
}
