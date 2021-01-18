use bandit::app;
use bandit::game::Game;


fn main() {
    let mut game = Game::new(10);
    game.start(None);
    for _ in 0..100 {
        game.play(0);
    }
    println!("{}", game.score());
    for _ in 0..1000 {
        game.play(1);
    }
    println!("{}", game.score());
    for _ in 0..10000 {
        game.play(2);
    }
    println!("{}", game.score());
    println!("{:?}", game.profiles());
    app::main()
}
