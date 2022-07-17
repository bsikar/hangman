use macroquad::prelude::*;
use rand::ChooseRandom;
use std::{fs::File, io::BufRead, io::BufReader};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

mod core;
use crate::hangman::core::screen::Screen;

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
    pub fn from_string(s: &str) -> Self {
        match s {
            "easy" => Self::Easy,
            "medium" => Self::Medium,
            "hard" => Self::Hard,
            _ => unreachable!(),
        }
    }

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

pub struct Hangman {
    num_wrong: u8,
    word: String,
    guess: Vec<Option<char>>,
    letters: Vec<Option<char>>,
    screen: Screen,
}

impl Hangman {
    pub fn new() -> Hangman {
        rand::srand(macroquad::miniquad::date::now() as _);
        let letters = ('A'..='Z').map(Some).collect();

        Hangman {
            letters,
            num_wrong: 0,
            word: "".to_string(),
            guess: vec![],
            screen: Screen::Start,
        }
    }

    pub fn play(&mut self) {
        if self.screen == Screen::Start {
            let difficulty = self.screen.get_difficulty();

            if let Some(difficulty) = difficulty {
                self.word = Self::get_word(difficulty);
                self.screen = Screen::Main;
            }
        } else if self.screen == Screen::Main {
            self.screen.draw_gallow();
            self.screen.draw_keyboard();

            let letter = self.screen.get_letter();
            if let Some(letter) = letter {}
        }
    }

    fn get_word(difficulty: Difficulty) -> String {
        let path = format!("{}/assets/", env!("CARGO_MANIFEST_DIR"));

        match difficulty {
            Difficulty::Easy => {
                let file = File::open(path + "easy-words.txt").unwrap();
                let list: Vec<String> = BufReader::new(file)
                    .lines()
                    .collect::<Result<_, _>>()
                    .unwrap();
                list.choose().unwrap().clone()
            }
            Difficulty::Medium => {
                let file = File::open(path + "medium-words.txt").unwrap();
                let list: Vec<String> = BufReader::new(file)
                    .lines()
                    .collect::<Result<_, _>>()
                    .unwrap();
                list.choose().unwrap().clone()
            }
            Difficulty::Hard => {
                let file = File::open(path + "hard-words.txt").unwrap();
                let list: Vec<String> = BufReader::new(file)
                    .lines()
                    .collect::<Result<_, _>>()
                    .unwrap();
                list.choose().unwrap().clone()
            }
        }
    }
}
