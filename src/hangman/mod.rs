use macroquad::prelude::*;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

mod core;
mod hangman;
pub use hangman::Hangman;

pub const TEXT_SIZE: f32 = 20.0; // smaller the number, the bigger the text
pub const TEXT_COLOR: Color = WHITE;
pub const BACKGROUND_COLOR: Color = BLACK;
pub const GALLOW_COLOR: Color = BEIGE;
pub const TITLE_TEXT: [&str; 2] = ["Welcome to Hangman!", "Select your difficulty below."];

#[derive(Debug, EnumCountMacro, EnumIter, Copy, Clone)]
pub enum Difficulty {
    Easy,   // length 3..=5
    Medium, // length 6..=9
    Hard,   // length 10+
}

impl Difficulty {
    pub fn as_str(&self) -> &str {
        match *self {
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
        }
    }

    pub fn as_color(&self) -> Color {
        match *self {
            Self::Easy => color_u8!(51, 112, 33, 255),
            Self::Medium => color_u8!(145, 138, 35, 255),
            Self::Hard => color_u8!(158, 43, 43, 255),
        }
    }
}
