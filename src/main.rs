use std::{fs::File, io::BufRead, io::BufReader};

mod screen;
use screen::Screen;

use macroquad::prelude::*;
use rand::ChooseRandom;

#[derive(Debug)]
pub enum Difficulty {
    Easy,   // length 3..=5
    Medium, // length 6..=9
    Hard,   // length 10+
}

#[derive(Debug)]
struct Hangman {
    num_wrong: u8,
    word: String,
    guess: String,
    screen: Screen,
}

impl Hangman {
    fn new() -> Hangman {
        Hangman {
            num_wrong: 0,
            word: "".to_string(),
            guess: "".to_string(),
            screen: Screen::Start,
        }
    }

    fn play(&mut self) {
        if self.screen == Screen::Start {
            let difficulty = self.screen.get_difficulty();

            if let Some(difficulty) = difficulty {
                self.word = Self::get_word(difficulty);
                self.screen = Screen::Main;
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

#[macroquad::main("Hangman")]
async fn main() {
    let mut hangman = Hangman::new();

    loop {
        println!("{hangman:?}");
        hangman.play();
        next_frame().await
    }
}
