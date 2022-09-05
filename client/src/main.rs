use std::{thread, time};
use crate::snake::game::Game;

mod snake;

fn main() {
    let mut game = Game::new();

    let duration = time::Duration::from_millis(300);

    while game.running() {
        game.update();
        game.draw();

        thread::sleep(duration);
    }
}
