use macroquad::prelude::next_frame;

mod hangman;
use hangman::Hangman;

#[macroquad::main("Hangman")]
async fn main() {
    let mut hangman = Hangman::new();

    loop {
        hangman.play();
        next_frame().await;

        // if esc was pressed break
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Escape) {
            break;
        }
    }
    println!("{hangman:#?}");
}
