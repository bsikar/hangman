use std::collections::HashSet;

use macroquad::prelude::*;
use strum::IntoEnumIterator;

use crate::hangman::{
    Difficulty, BACKGROUND_COLOR, BUTTON_GRAY, BUTTON_RED, TEXT_COLOR, TEXT_SIZE, TITLE_TEXT,
};

use super::button::Button;
use crate::hangman::core::gallow::Gallow;
use crate::hangman::core::person::Person;

#[derive(PartialEq, Debug)]
pub struct Screen {
    person: Person,
    gallow: Gallow,
    pub screen_type: ScreenType,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ScreenType {
    Start,
    Main,
    End,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            person: Person::new(),
            gallow: Gallow::new(),
            screen_type: ScreenType::Start,
        }
    }

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

            let button = Button::new(
                (x, y),
                (w, w),
                difficulty.as_str().to_string(),
                difficulty.as_color(),
            );
            button.draw();

            if let Some(x) = button.was_pressed() {
                return Some(Difficulty::from_string(x.as_str()));
            }
        }

        None
    }

    pub fn draw_gallow(&mut self) {
        self.gallow.draw();
    }

    pub fn draw_keyboard(&self, letters: &HashSet<char>) -> Option<char> {
        let characters = [
            vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
            vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
            vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
        ];

        // gallow y and h bottom cords
        let y = screen_height() / 6.0 + screen_height() / 2.5;
        let h = screen_height() / 40.0;
        let gallow_height = y + h;
        let top_gap = (gallow_height / 4.0) / 2.0;
        let spacing = 1.5;

        for (o, row) in characters.iter().enumerate() {
            for (i, c) in row.iter().enumerate() {
                let w = if screen_height() > screen_width() {
                    screen_width() / 15.0
                } else {
                    screen_height() / 15.0
                };
                let x = (screen_width() / 2.0) - (w * row.len() as f32 / 2.0 * spacing)
                    + (w * (i as f32 + 0.14) * spacing);
                let y = top_gap + gallow_height + (o as f32 * w) + (o as f32 * top_gap / 4.0);

                let button = Button::new(
                    (x, y),
                    (w, w),
                    c.to_string(),
                    if letters.contains(c) {
                        BUTTON_RED
                    } else {
                        BUTTON_GRAY
                    },
                );
                button.draw();

                if let Some(c) = button.was_pressed() {
                    return c.chars().next();
                } else if let Some(c) = get_char_pressed() {
                    return Some(c.to_ascii_lowercase());
                }
            }
        }
        None
    }

    pub fn draw_word(&self, guess: &[Option<char>], word: String) {
        let bottom_of_gallow = self.gallow.parts.get("bar_on_bottom").unwrap();
        let y = bottom_of_gallow.y + bottom_of_gallow.h + 10.0;

        let text_size_ratio = if screen_height() > screen_width() {
            screen_width() / TEXT_SIZE
        } else {
            screen_height() / TEXT_SIZE
        };

        let mut text = String::new();
        for c in word.chars() {
            if guess.contains(&Some(c)) {
                text.push(c);
            } else {
                text.push('_');
            }
        }

        let text_size = measure_text(&text, None, text_size_ratio as u16, 1.0);

        draw_text(
            &text,
            screen_width() / 2.0 - text_size.width / 2.0,
            y,
            text_size_ratio,
            TEXT_COLOR,
        );
    }

    pub fn draw_end_screen(&self, did_win: bool) -> bool {
        let text_size_ratio = if screen_height() > screen_width() {
            screen_width() / TEXT_SIZE
        } else {
            screen_height() / TEXT_SIZE
        };

        let text = "Game Over!";
        let text_size = measure_text(text, None, (text_size_ratio * 4.0) as u16, 1.0);

        draw_text(
            text,
            screen_width() / 2.0 - text_size.width / 2.0,
            (screen_height() / 2.0 - text_size.height / 2.0) - 3.0 * text_size_ratio,
            text_size_ratio * 4.0,
            TEXT_COLOR,
        );

        let text = if did_win { "You Won!" } else { "You Lost!" };
        let text_size = measure_text(text, None, (text_size_ratio * 3.0) as u16, 1.0);

        draw_text(
            text,
            screen_width() / 2.0 - text_size.width / 2.0,
            (screen_height() / 2.0 - text_size.height / 2.0) - 3.0 * text_size_ratio
                + text_size_ratio * 2.0,
            text_size_ratio * 3.0,
            TEXT_COLOR,
        );

        let spacing = screen_width() / 5.0;
        let gap = 30.0;
        let x = spacing * 1.0 + (gap / 2.0);
        let y = screen_height() / 2.0;
        let w = spacing - gap;

        let button = Button::new(
            (x, y),
            (w, w),
            "Play Again".to_string(),
            Difficulty::Easy.as_color(),
        );
        button.draw();

        if button.was_pressed().is_some() {
            return true;
        }

        let spacing = screen_width() / 5.0;
        let gap = 30.0;
        let x = spacing * 3.0 + (gap / 2.0);
        let y = screen_height() / 2.0;
        let w = spacing - gap;

        let button = Button::new(
            (x, y),
            (w, w),
            "Quit Game".to_string(),
            Difficulty::Hard.as_color(),
        );
        button.draw();

        if button.was_pressed().is_some() {
            std::process::exit(0);
        }

        false
    }

    pub fn draw_person(&mut self, num_wrong: usize) {
        self.person.draw(num_wrong, &self.gallow);
    }
}
