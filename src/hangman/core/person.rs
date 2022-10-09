use crate::hangman::HANGMAN_COLOR;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Part {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(PartialEq, Debug)]
pub struct Person {
    pub parts: HashMap<String, Part>,
}

impl Person {
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
        }
    }

    pub fn draw(&mut self, num_wrong: usize) {
        // list of function pointers
        let draw_list = [
            Self::draw_head,
            Self::draw_body,
            Self::draw_left_arm,
            Self::draw_right_arm,
            Self::draw_left_leg,
            Self::draw_right_leg,
        ];

        // XXX tmp for testing
        for i in draw_list {
            (i)(self);
        }

        for i in draw_list.iter().take(num_wrong) {
            (i)(self);
        }
    }

    // 1
    pub fn draw_head(&mut self) {
        // down on right
        let bar_down_x = screen_width() / 2.5 + screen_width() / 40.0;
        let bar_down_y = screen_height() / 6.0 + screen_height() / 15.0;

        let x = bar_down_x - screen_width() / 80.0;
        let y = bar_down_y + screen_width() / 30.0;
        let r = if screen_height() > screen_width() {
            screen_width() / 20.0
        } else {
            screen_height() / 20.0
        };
        draw_circle(x, y, r, HANGMAN_COLOR);

        self.parts
            .insert("head".to_string(), Part { x, y, w: r, h: r });
    }

    // 2
    pub fn draw_body(&mut self) {
        let x = screen_width() / 2.5;
        let w = screen_width() / 40.0;
        let bar_down_y = screen_height() / 6.0 + screen_height() / 15.0;

        let y = bar_down_y;
        let h = screen_height() / 5.0;
        draw_rectangle(x, y, w, h, HANGMAN_COLOR);

        self.parts.insert("body".to_string(), Part { x, y, w, h });
    }

    // 3
    pub fn draw_left_arm(&mut self) {}

    // 4
    pub fn draw_right_arm(&mut self) {}

    // 5
    pub fn draw_left_leg(&mut self) {
        let x1 = screen_width() / 2.45;
        let y1 = screen_height() / 9.0 + screen_height() / 15.0 + screen_height() / 4.0;
        let w = screen_width() / 40.0;

        let x2 = screen_width() / 3.0;
        let y2 = screen_height() / 1.75 - screen_height() / 15.0;

        draw_line(x1, y1, x2, y2, w, HANGMAN_COLOR);

        self.parts.insert(
            "left_leg".to_string(),
            Part {
                x: x1,
                y: y1,
                w: x2 - x1,
                h: y2 - y1,
            },
        );
    }

    // 6
    pub fn draw_right_leg(&mut self) {
        let x1 = screen_width() / 2.45;
        let y1 = screen_height() / 9.0 + screen_height() / 15.0 + screen_height() / 4.0;
        let w = screen_width() / 40.0;

        let x2 = screen_width() / 2.0;
        let y2 = screen_height() / 1.75 - screen_height() / 15.0;

        draw_line(x1, y1, x2, y2, w, HANGMAN_COLOR);

        self.parts.insert(
            "right_leg".to_string(),
            Part {
                x: x1,
                y: y1,
                w: x2 - x1,
                h: y2 - y1,
            },
        );
    }
}
