use macroquad::prelude::*;
use rand::ChooseRandom;
use std::{fs::File, io::BufRead, io::BufReader};

use super::{core::screen::Screen, Difficulty};

#[derive(Debug)]
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
        let letters = ('A'..='Z').map(|c| Some(c)).collect();

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
            if letter.is_some() {
                let letter = letter.unwrap();
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
