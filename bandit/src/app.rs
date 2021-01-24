
use std::sync::Mutex;
use crate::game::Game;
use rocket::State;

#[get("/")]
fn desc() -> &'static str {
    "You can play games"
}

#[get("/score")]
fn score(state: State<Mutex<Game>>) -> String {
    let game = state.lock().unwrap();
    format!(
        "Your Score: {}\nPlay Count: {}\nMean Score: {}",
        game.score(),
        game.play_count(),
        game.score() / (game.play_count() as f64)
    ) 
}

#[get("/reset")]
fn reset(state: State<Mutex<Game>>) -> String {
    unimplemented!()
}

#[get("/start")]
fn start(state: State<Mutex<Game>>) -> String {
    let mut game = state.lock().unwrap();
    game.start(Option::None);
    "Game start".to_string()
}

#[get("/play/<index>")]
fn play(index: usize, state: State<Mutex<Game>>) -> String {
    let mut game = state.lock().unwrap();
    match game.play(index) {
        Ok(val) => format!("Won {}", val),
        Err(val) => val 
    }
}


pub fn main() {
    rocket::ignite().mount(
        "/game",
        routes![
            desc, 
            start,
            play, 
            score,
            reset,
        ]
    )
    .manage(Mutex::new(Game::new(10)))
    .launch();
}
