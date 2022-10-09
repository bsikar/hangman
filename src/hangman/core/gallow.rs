use macroquad::prelude::*;

use std::collections::HashMap;

use crate::hangman::GALLOW_COLOR;

#[derive(PartialEq, Debug)]
pub struct Part {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(PartialEq, Debug)]
pub struct Gallow {
    pub parts: HashMap<String, Part>,
}

impl Gallow {
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
        }
    }

    pub fn draw(&mut self) {
        self.draw_down_on_right();
        self.draw_down_on_left();
        self.draw_bar_on_top();
        self.draw_bar_on_bottom();
    }

    fn draw_down_on_right(&mut self) {
        let x = screen_width() / 2.5;
        let y = screen_height() / 6.0;
        let w = screen_width() / 40.0;
        let h = screen_height() / 15.0;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);

        self.parts
            .insert("down_on_right".to_string(), Part { x, y, w, h });
    }

    pub fn draw_down_on_left(&mut self) {
        let x = screen_width() / 7.0;
        let y = screen_height() / 6.0;
        let w = screen_width() / 40.0;
        let h = screen_height() / 2.5;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);

        self.parts
            .insert("down_on_left".to_string(), Part { x, y, w, h });
    }

    pub fn draw_bar_on_top(&mut self) {
        let x = screen_width() / 7.0;
        let y = screen_height() / 6.0;
        let w = screen_width() / 2.5 + screen_width() / 40.0 - screen_width() / 7.0;
        let h = screen_height() / 40.0;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);

        self.parts
            .insert("bar_on_top".to_string(), Part { x, y, w, h });
    }

    pub fn draw_bar_on_bottom(&mut self) {
        let x = screen_width() / 7.0 - screen_width() / 20.0;
        let y = screen_height() / 6.0 + screen_height() / 2.5;
        let w = screen_width() / 40.0 + screen_width() / 10.0;
        let h = screen_height() / 40.0;
        draw_rectangle(x, y, w, h, GALLOW_COLOR);

        self.parts
            .insert("bar_on_bottom".to_string(), Part { x, y, w, h });
    }
}
