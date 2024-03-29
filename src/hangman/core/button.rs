use macroquad::prelude::*;

use crate::hangman::{TEXT_COLOR, TEXT_SIZE};

pub struct Button {
    coordinates: (f32, f32),
    dimensions: (f32, f32),
    text: String,
    color: Color,
}

impl Button {
    pub fn new(
        coordinates: (f32, f32),
        dimensions: (f32, f32),
        text: String,
        color: Color,
    ) -> Button {
        Button {
            coordinates,
            dimensions,
            text,
            color,
        }
    }

    pub fn draw(&self) {
        let text_size_ratio = if screen_height() > screen_width() {
            screen_width() / TEXT_SIZE
        } else {
            screen_height() / TEXT_SIZE
        };
        draw_rectangle(
            self.coordinates.0,
            self.coordinates.1,
            self.dimensions.0,
            self.dimensions.1,
            self.color,
        );
        let text_size = measure_text(&self.text, None, text_size_ratio as u16, 1.0);

        // if the text is too big split it into two lines
        let words = self.text.split(' ').collect::<Vec<&str>>();
        if text_size.width > self.dimensions.0 {
            // we can assume that there are only two words
            let distance = self.dimensions.0 / 2.0;
            let text_size = measure_text(words[0], None, text_size_ratio as u16, 1.0);
            draw_text(
                words[0],
                self.coordinates.0 + distance - text_size.width / 2.0,
                (self.coordinates.1 + distance) - text_size.height / 2.0,
                text_size_ratio,
                TEXT_COLOR,
            );
            if words.len() > 1 {
                let text_size = measure_text(words[1], None, text_size_ratio as u16, 1.0);
                draw_text(
                    words[1],
                    self.coordinates.0 + distance - text_size.width / 2.0,
                    self.coordinates.1 + distance + text_size_ratio,
                    text_size_ratio,
                    TEXT_COLOR,
                );
            }
        } else {
            draw_text(
                &self.text,
                self.coordinates.0 + (self.dimensions.0 / 2.0 - text_size.width / 2.0),
                self.coordinates.1 + (self.dimensions.1 / 2.0 + text_size.height / 2.0),
                text_size_ratio,
                TEXT_COLOR,
            );
        }
    }

    pub fn was_pressed(&self) -> Option<String> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            if x >= self.coordinates.0
                && y >= self.coordinates.1
                && x <= self.coordinates.0 + self.dimensions.0
                && y <= self.coordinates.1 + self.dimensions.1
            {
                return Some(self.text.clone());
            }
        }

        None
    }
}
