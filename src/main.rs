use rusty_snake::{game, run};

fn main() {
    let game = game::Game::new().unwrap();
    run(game);
}
