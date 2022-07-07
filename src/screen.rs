use crate::Difficulty;
use macroquad::prelude::*;

const TEXT_SIZE: f32 = 20.0; // smaller the number, the bigger the text
const TEXT_COLOR: Color = WHITE;
const BACKGROUND_COLOR: Color = BLACK;
const BUTTON_COLORS: [Color; 3] = [
    color_u8!(51, 112, 33, 255),
    color_u8!(189, 170, 26, 255),
    color_u8!(140, 32, 32, 255),
];
const BUTTON_NAMES: [&'static str; 3] = ["easy", "medium", "hard"];
const TITLE_TEXT: [&'static str; 2] = ["Welcome to Hangman!", "Select your difficulty below."];

#[derive(Debug, PartialEq)]
pub enum Screen {
    Start,
    Main,
}

impl Screen {
    pub fn get_difficulty(&self) -> Option<Difficulty> {
        clear_background(BACKGROUND_COLOR);

        let text_size_ratio = if screen_height() > screen_width() {
            screen_width() / TEXT_SIZE
        } else {
            screen_height() / TEXT_SIZE
        };

        for i in 0..(TITLE_TEXT.len()) {
            let text_size = measure_text(TITLE_TEXT[i], None, text_size_ratio as u16, 1.0);

            draw_text(
                TITLE_TEXT[i],
                screen_width() / 2.0 - text_size.width / 2.0,
                (screen_height() / 2.0 - text_size.height / 2.0) - 3.0 * text_size_ratio
                    + (i as f32 * text_size_ratio),
                text_size_ratio,
                TEXT_COLOR,
            );
        }

        // draw boxes for each difficulty
        let spacing = screen_width() / 5.0;

        for i in 0..(BUTTON_NAMES.len()) {
            let gap = 30.0;
            let x = spacing * (i as f32 + 1.0) + (gap / 2.0);
            let y = screen_height() / 2.0;
            let w = spacing - gap;
            draw_rectangle(x, y, w, w, BUTTON_COLORS[i]);

            let text_size = measure_text(BUTTON_NAMES[i], None, text_size_ratio as u16, 1.0);
            draw_text(
                BUTTON_NAMES[i],
                x + (w / 2.0 - text_size.width / 2.0),
                y + (w / 2.0),
                text_size_ratio,
                TEXT_COLOR,
            );
        }

        // check if user has pressed a button

        // return the difficulty if the user has pressed a button

        // otherwise return None
        None
    }
}
