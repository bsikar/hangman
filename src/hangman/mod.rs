use macroquad::prelude::*;
use rand::ChooseRandom;
use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

mod core;
use crate::hangman::core::screen::{Screen, ScreenType};

pub const TEXT_SIZE: f32 = 20.0; // smaller the number, the bigger the text
pub const TEXT_COLOR: Color = color_u8!(197, 194, 154, 255);
pub const BACKGROUND_COLOR: Color = color_u8!(23, 23, 23, 255);
pub const GALLOW_COLOR: Color = color_u8!(156, 119, 86, 255);
pub const EASY_GREEN: Color = color_u8!(123, 119, 86, 255);
pub const MEDIUM_YELLOW: Color = color_u8!(156, 119, 86, 255);
pub const HARD_RED: Color = color_u8!(156, 85, 86, 255);
pub const BUTTON_RED: Color = HARD_RED;
pub const BUTTON_GRAY: Color = color_u8!(40, 37, 38, 255);
pub const HANGMAN_COLOR: Color = color_u8!(111, 108, 90, 255);
pub const TITLE_TEXT: [&str; 2] = ["Welcome to Hangman!", "Select your difficulty below."];
pub const MAX_WRONG: usize = 6;

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
            Self::Easy => EASY_GREEN,
            Self::Medium => MEDIUM_YELLOW,
            Self::Hard => HARD_RED,
        }
    }
}

#[derive(Debug)]
pub struct Hangman {
    letters_wrong: Vec<char>,
    word: String,
    guess: Vec<Option<char>>,
    letters: HashSet<char>,
    screen: Screen,
}

impl Hangman {
    pub fn new() -> Hangman {
        rand::srand(macroquad::miniquad::date::now() as _);
        let letters = ('a'..='z').collect();

        Hangman {
            letters,
            letters_wrong: vec![],
            word: "".to_string(),
            guess: vec![],
            screen: Screen::new(),
        }
    }

    pub fn play(&mut self) {
        self.screen.screen_type = ScreenType::Main; // XXX tmp for testing
        match self.screen.screen_type {
            ScreenType::Start => {
                let difficulty = self.screen.get_difficulty();

                if let Some(difficulty) = difficulty {
                    self.word = Self::get_word(difficulty);
                    self.screen.screen_type = ScreenType::Main;
                }
            }
            ScreenType::Main => {
                clear_background(BACKGROUND_COLOR);
                self.screen.draw_gallow();
                self.screen.draw_person(self.letters_wrong.len());
                let letter = self.screen.draw_keyboard(&self.letters);

                if let Some(letter) = letter {
                    if self.letters.contains(&letter) {
                        self.letters.remove(&letter);
                        if self.word.contains(letter) {
                            self.guess.push(Some(letter));
                        } else {
                            self.letters_wrong.push(letter);

                            if self.letters_wrong.len() == MAX_WRONG {
                                self.screen.screen_type = ScreenType::End;
                            }
                        }
                    }
                }
            }
            ScreenType::End => {
                self.screen.draw_gallow();
                let play_again = self.screen.draw_end_screen();
                if play_again {
                    *self = Hangman::new();
                }
            }
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
