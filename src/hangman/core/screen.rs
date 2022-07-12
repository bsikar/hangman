use macroquad::prelude::*;
use strum::IntoEnumIterator;

use crate::hangman::{Difficulty, BACKGROUND_COLOR, TEXT_COLOR, TEXT_SIZE, TITLE_TEXT};

use super::button::Button;

#[derive(Debug, PartialEq)]
pub enum Screen {
    Start,
    Main,
    End,
}

impl Screen {
    pub fn get_difficulty(&self) -> Option<Difficulty> {
        clear_background(BACKGROUND_COLOR);

        let text_size_ratio = if screen_height() > screen_width() {
            screen_width() / TEXT_SIZE
        } else {
            screen_height() / TEXT_SIZE
        };

        for (i, text) in TITLE_TEXT.iter().enumerate() {
            let text_size = measure_text(text, None, text_size_ratio as u16, 1.0);

            draw_text(
                text,
                screen_width() / 2.0 - text_size.width / 2.0,
                (screen_height() / 2.0 - text_size.height / 2.0) - 3.0 * text_size_ratio
                    + (i as f32 * text_size_ratio),
                text_size_ratio,
                TEXT_COLOR,
            );
        }

        for (i, difficulty) in Difficulty::iter().enumerate() {
            let spacing = screen_width() / 5.0;
            let gap = 30.0;
            let x = spacing * (i as f32 + 1.0) + (gap / 2.0);
            let y = screen_height() / 2.0;
            let w = spacing - gap;

            let button = Button::new((x, y), (w, w), difficulty);
            button.draw();

            if let Some(x) = button.was_pressed() {
                return Some(x);
            }
        }

        None
    }

    pub fn get_letter(&self) -> Option<char> {
        Some('a')
    }
}
