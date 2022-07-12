use macroquad::prelude::{
    draw_rectangle, draw_text, is_mouse_button_pressed, measure_text, mouse_position,
    screen_height, screen_width, MouseButton,
};

use crate::hangman::{Difficulty, TEXT_COLOR, TEXT_SIZE};

#[derive(Debug)]
pub struct Button {
    coordinates: (f32, f32),
    dimensions: (f32, f32),
    difficulty: Difficulty,
}

impl Button {
    pub fn new(coordinates: (f32, f32), dimensions: (f32, f32), difficulty: Difficulty) -> Button {
        Button {
            coordinates,
            dimensions,
            difficulty,
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
            self.difficulty.as_color(),
        );
        let text_size = measure_text(self.difficulty.as_str(), None, text_size_ratio as u16, 1.0);
        draw_text(
            self.difficulty.as_str(),
            self.coordinates.0 + (self.dimensions.0 / 2.0 - text_size.width / 2.0),
            self.coordinates.1 + (self.dimensions.1 / 2.0 + text_size.height / 2.0),
            text_size_ratio,
            TEXT_COLOR,
        );
    }

    pub fn was_pressed(&self) -> Option<Difficulty> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            if x >= self.coordinates.0
                && y >= self.coordinates.1
                && x <= self.coordinates.0 + self.dimensions.0
                && y <= self.coordinates.1 + self.dimensions.1
            {
                return Some(self.difficulty);
            }
        }

        None
    }
}
