use macroquad::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

use crate::GameAssets;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, EnumCountMacro, Hash)]
#[repr(u8)]
pub enum CardRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardRank {
    pub const FONT_SIZE: u16 = 30;

    pub fn draw(self, assets: &GameAssets, x: f32, y: f32, color: Color) {
        let text: &'static str = self.into();

        let font = &assets.font_regular;

        let text_dimensions = measure_text(text, font.into(), Self::FONT_SIZE, 1.0);

        let x = x + 15.0 - text_dimensions.width * 0.5;
        let y = y + 15.0 + text_dimensions.height * 0.5;

        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: font.into(),
                font_size: Self::FONT_SIZE,
                color,
                ..Default::default()
            },
        );
    }

    pub const fn from_u8(value: u8) -> Option<Self> {
        if value < Self::COUNT as u8 {
            // SAFETY: The value is guaranteed to be a valid variant discriminant.
            Some(unsafe { std::mem::transmute(value) })
        } else {
            None
        }
    }
}

impl From<CardRank> for &'static str {
    fn from(rank: CardRank) -> Self {
        match rank {
            CardRank::Two => "2",
            CardRank::Three => "3",
            CardRank::Four => "4",
            CardRank::Five => "5",
            CardRank::Six => "6",
            CardRank::Seven => "7",
            CardRank::Eight => "8",
            CardRank::Nine => "9",
            CardRank::Ten => "10",
            CardRank::Jack => "J",
            CardRank::Queen => "Q",
            CardRank::King => "K",
            CardRank::Ace => "A",
        }
    }
}
