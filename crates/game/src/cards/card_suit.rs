use crate::*;
use macroquad::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
#[repr(u8)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl CardSuit {
    const FONT_SIZE: u16 = 40;

    pub fn draw(self, assets: &GameAssets, x: f32, y: f32) {
        let color = self.color();

        let offset_x = Card::WIDTH * 0.5;
        let offset_y = Card::HEIGHT * 0.5;

        let text = self.into();
        let font = &assets.font_regular;

        let text_dimensions = measure_text(text, font.into(), Self::FONT_SIZE, 1.0);

        let x = x + offset_x - text_dimensions.width * 0.5;
        let y = y + offset_y + text_dimensions.height * 0.5;

        // set font
        draw_text_ex(
            self.into(),
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

    pub fn color(&self) -> Color {
        match self {
            Self::Hearts | Self::Diamonds => Color::from_rgba(255, 0, 0, 255),
            Self::Clubs | Self::Spades => Color::from_rgba(0, 0, 0, 255),
        }
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

impl From<CardSuit> for &'static str {
    fn from(suit: CardSuit) -> Self {
        match suit {
            CardSuit::Hearts => "♥",
            CardSuit::Diamonds => "♦",
            CardSuit::Clubs => "♣",
            CardSuit::Spades => "♠",
        }
    }
}
