use macroquad::prelude::*;
use strum::IntoEnumIterator;

use crate::hangman::{
    Difficulty, BACKGROUND_COLOR, GALLOW_COLOR, TEXT_COLOR, TEXT_SIZE, TITLE_TEXT,
};

use super::button::Button;

#[derive(PartialEq)]
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

    pub fn get_letter(&self) -> Option<char> {
        Some('a')
    }

    pub fn draw_gallow(&self) {
        // down on right
        let x = screen_width() / 2.5;
        let y = screen_height() / 6.0;
        let w = screen_width() / 40.0;
        let h = screen_height() / 15.0;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);

        // down on left
        let x = screen_width() / 7.0;
        let y = screen_height() / 6.0;
        let w = screen_width() / 40.0;
        let h = screen_height() / 2.5;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);

        // bar on top
        let x = screen_width() / 7.0;
        let y = screen_height() / 6.0;
        let w = screen_width() / 2.5 + screen_width() / 40.0 - screen_width() / 7.0;
        let h = screen_height() / 40.0;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);

        // bar on bottom
        let x = screen_width() / 7.0 - screen_width() / 20.0;
        let y = screen_height() / 6.0 + screen_height() / 2.5;
        let w = screen_width() / 40.0 + screen_width() / 10.0;
        let h = screen_height() / 40.0;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);
    }

    pub fn draw_keyboard(&self) {
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

                let button = Button::new((x, y), (w, w), c.to_string(), RED);
                button.draw();

                if let Some(x) = button.was_pressed() {
                    println!("{x}");
                }
            }
        }
    }
}
