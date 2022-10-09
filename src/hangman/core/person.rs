use crate::hangman::core::gallow::{self, Gallow};
use crate::hangman::HANGMAN_COLOR;
use macroquad::prelude::*;
use std::collections::HashMap;

const CHANGE_RED_TIME: f64 = 3.0;

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
    time_since_wrong: f64,
    make_red: bool,
}

impl Person {
    pub fn new() -> Self {
        Self {
            parts: HashMap::new(),
            time_since_wrong: 0.0,
            make_red: false,
        }
    }

    pub fn draw(&mut self, num_wrong: usize, gallow: &Gallow) {
        // list of function pointers
        let draw_list = [
            Self::draw_head,
            Self::draw_body,
            Self::draw_left_arm,
            Self::draw_right_arm,
            Self::draw_left_leg,
            Self::draw_right_leg,
            Self::draw_left_eye,
            Self::draw_right_eye,
            Self::draw_mouth,
        ];

        for i in draw_list.iter().take(num_wrong) {
            (i)(self, gallow.parts.clone());
        }
    }

    // 1
    fn draw_head(&mut self, parts: HashMap<String, gallow::Part>) {
        let top_of_gallow = parts.get("down_on_right").unwrap();
        let w = if screen_width() > screen_height() {
            screen_height() / 20.0
        } else {
            screen_width() / 20.0
        };
        let x = top_of_gallow.x + top_of_gallow.w / 2.0;
        let y = top_of_gallow.y + top_of_gallow.h + w / 2.0;

        draw_circle(x, y, w, HANGMAN_COLOR);
        self.parts
            .insert("head".to_string(), Part { x, y, w, h: w });
    }

    // 2
    fn draw_body(&mut self, _parts: HashMap<String, gallow::Part>) {
        let head = self.parts.get("head").unwrap();

        let x = head.x - head.w / 4.0;
        let y = head.y + head.h / 2.0;
        let w = head.w / 2.0;
        let h = head.w * 2.0;

        draw_rectangle(x, y, w, h, HANGMAN_COLOR);
        self.parts.insert("body".to_string(), Part { x, y, w, h });
    }

    // 3
    fn draw_left_arm(&mut self, _parts: HashMap<String, gallow::Part>) {
        // draw a rectangle from the body to the left
        let body = self.parts.get("body").unwrap();

        let x = body.x - body.w;
        let y = body.y + body.h / 2.0;
        let w = body.w;
        let h = body.h / 2.0;

        draw_rectangle(x, y, w, h, HANGMAN_COLOR);
        self.parts
            .insert("left_arm".to_string(), Part { x, y, w, h });
    }

    // 4
    fn draw_right_arm(&mut self, _parts: HashMap<String, gallow::Part>) {
        // draw a rectangle from the body to the right
        let body = self.parts.get("body").unwrap();

        let x = body.x + body.w;
        let y = body.y + body.h / 2.0;
        let w = body.w;
        let h = body.h / 2.0;

        draw_rectangle(x, y, w, h, HANGMAN_COLOR);
        self.parts
            .insert("right_arm".to_string(), Part { x, y, w, h });
    }

    // 5
    fn draw_left_leg(&mut self, _parts: HashMap<String, gallow::Part>) {
        // draw a rectangle from the body to the left
        let body = self.parts.get("body").unwrap();

        let x = body.x - body.w / 2.0;
        let y = body.y + body.h;
        let w = body.w / 2.0;
        let h = body.h / 2.0;

        draw_rectangle(x, y, w, h, HANGMAN_COLOR);
        self.parts
            .insert("left_leg".to_string(), Part { x, y, w, h });
    }

    // 6
    fn draw_right_leg(&mut self, _parts: HashMap<String, gallow::Part>) {
        // draw a rectangle from the body to the right
        let body = self.parts.get("body").unwrap();

        let x = body.x + body.w;
        let y = body.y + body.h;
        let w = body.w / 2.0;
        let h = body.h / 2.0;

        draw_rectangle(x, y, w, h, HANGMAN_COLOR);
        self.parts
            .insert("right_leg".to_string(), Part { x, y, w, h });
    }

    // 7
    fn draw_left_eye(&mut self, _parts: HashMap<String, gallow::Part>) {
        // draw a rectangle from the body to the left
        let head = self.parts.get("head").unwrap();

        let x = head.x - head.w / 2.0;
        let y = head.y - head.h / 2.0;
        let w = head.w / 2.0;
        let h = head.h / 2.0;

        if self.make_red {
            draw_circle(x, y + w / 2.0, w / 2.0, RED);
        } else {
            draw_circle(x, y + w / 2.0, w / 2.0, WHITE);
        }

        self.parts
            .insert("left_eye".to_string(), Part { x, y, w, h });
    }

    // 8
    fn draw_right_eye(&mut self, _parts: HashMap<String, gallow::Part>) {
        // draw a rectangle from the body to the right
        let head = self.parts.get("head").unwrap();

        let x = head.x + head.w / 2.0;
        let y = head.y - head.h / 2.0;
        let w = head.w / 2.0;
        let h = head.h / 2.0;

        if self.make_red {
            draw_circle(x, y + w / 2.0, w / 2.0, RED);
        } else {
            draw_circle(x, y + w / 2.0, w / 2.0, WHITE);
        }

        self.parts
            .insert("right_eye".to_string(), Part { x, y, w, h });
    }

    // 9
    fn draw_mouth(&mut self, _parts: HashMap<String, gallow::Part>) {
        // draw a mouth by drawing 2 circles offset from the head
        let head = self.parts.get("head").unwrap();

        let x = head.x;
        let y = head.y + head.h / 2.0;
        let w = head.w / 3.0;
        if self.time_since_wrong == 0.0 {
            self.time_since_wrong = get_time();
        } else if get_time() - self.time_since_wrong > CHANGE_RED_TIME {
            draw_circle(x, y, w, RED);
            self.make_red = true;
        } else {
            draw_circle(x, y, w, WHITE);
        }

        let y = head.y + head.h / 2.0 - w / 2.0;
        let w = head.w / 3.0;

        self.parts
            .insert("mouth".to_string(), Part { x, y, w, h: w });
        draw_circle(x, y, w, HANGMAN_COLOR);
    }
}
