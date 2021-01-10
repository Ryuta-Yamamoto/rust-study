use std::{usize, vec::Vec};
use std::any::Any;
use statrs::distribution::Beta;
use rand::random;
use rand::distributions::Distribution;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, thread_rng};


trait Replayable {
    fn as_any(&self) -> &Any;
    fn initialize(&mut self);
    fn play(&mut self) -> f64;
    fn profile(&self) -> f64;
}


#[derive(Debug, Clone)]
struct BinarySlot {
    prob: f64,
    rng: StdRng,
    seed: u64,
}

impl Replayable for BinarySlot {
    fn as_any(&self) -> &Any {
        self
    }
    fn initialize(&mut self) {
        self.rng = StdRng::seed_from_u64(self.seed)
    }
    fn play(&mut self) -> f64 {
        (self.rng.gen::<f64>() < self.prob) as u32 as f64
    }
    fn profile(&self) -> f64 {
        self.prob
    }
}

impl BinarySlot {
    fn new() -> BinarySlot {
        let seed: u64 = random();
        BinarySlot {
            prob: random(),
            rng: StdRng::seed_from_u64(seed),
            seed: seed
        }
    }
}

trait Factory<T: Replayable> {
    fn gen(&mut self) -> T; 
}

trait Storage<T: Replayable + Clone + 'static> {
    fn hist(&mut self) -> &Vec<Box<dyn Replayable>>;
    fn nth(&mut self, n: usize) -> Option<T> {
        if let Some(val) = self.hist().get(n) {
            Some(val.as_any().downcast_ref::<T>().unwrap().clone());
        }
        None
    }
}

struct SlotRepository {
    storage: Vec<Box<dyn Replayable>>
}

impl SlotRepository {
    fn new() -> SlotRepository {
        SlotRepository{ storage: vec![] }
    }
}

impl Factory<BinarySlot> for SlotRepository {
    fn gen(&mut self) -> BinarySlot {
        self.storage.push(Box::new(BinarySlot::new()));
        self.storage.last_mut().unwrap().as_any().downcast_ref::<BinarySlot>().unwrap().clone()
    }
}

impl Storage<BinarySlot> for SlotRepository {
    fn hist(&mut self) -> &Vec<Box<dyn Replayable>> {
        &mut self.storage
    }
    fn nth(&mut self, n: usize) -> Option<BinarySlot> {
        if let Some(val) = self.storage.get(n) {
            Some(val.as_any().downcast_ref::<BinarySlot>().unwrap().clone());
        }
        None
    }
}

struct SlotMachine {
    slot: Box<dyn Replayable>,
    rewards: Vec<f64>,
    repository: SlotRepository,
}

impl SlotMachine {
    fn new() -> SlotMachine {
        let mut repository = SlotRepository::new();
        let slot = repository.gen();
        SlotMachine {
            slot: Box::new(slot),
            rewards: vec![],
            repository
        }
    }
    fn play(&mut self) -> f64 {
        let v = self.slot.play();
        self.rewards.push(v);
        v
    }    
    fn set(&mut self, slot: Box<dyn Replayable>) {
        self.slot = slot;
    }
    fn reset(&mut self) {
        self.slot.initialize()
    }
    fn set_nth(&mut self, index: usize) -> Result<(), String> {
        let new = self.repository.nth(index);
        match new {
            Some(slot) => {
                self.set(Box::new(slot));
                Ok(())
            }
            None => Err("Index out of range".to_string())
        }
    }
}

enum State {
    End,
    Playing { cnt: usize, max: Option<usize>},
}

impl State {
    fn new(n_games: Option<usize>) -> State {
        match n_games {                
            Some(n @ 1..=usize::MAX) => State::Playing { cnt: 0, max: Some(n) },
            Some(_) => State::End,
            None => State::Playing { cnt: 0, max: None }
        } 
    }
    fn play(&self) -> State {
        match *self {
            State::Playing {cnt, max: Some(max)} => {
                if cnt < max {
                    State::Playing { cnt: cnt + 1, max: Some(max)}
                } else { State::End }
            }
            State::Playing { cnt, max: None } => State::Playing { cnt, max: None },
            _ => panic!()
        }
    }
}

struct Game {
    slot_machines: Vec<SlotMachine>,
    state: State,
    scores: Vec<f64>
}

impl Game {
    fn new(n_machines: usize) -> Game {
        let mut slot_machines = Vec::with_capacity(n_machines);
        for _ in 0..n_machines {
            slot_machines.push(SlotMachine::new())
        }
        Game {
            slot_machines,
            state: State::End,
            scores: Vec::new(),
        }
    }
    fn start(&mut self, n_games: Option<usize>) {
        self.state = State::new(n_games)
    }
    fn play(&mut self, index: usize) -> Result<f64, String> {
        let max_index = self.slot_machines.len();
        if let State::End = self.state {
            return Err("A game is not started. Please start a game.".to_string())
        }
        match self.slot_machines.get_mut(index) {
            Some(slot) => {
                let state = self.state.play();
                let reword = slot.play();
                self.scores.push(reword);
                Ok(reword)
            }
            None => Err("Index out of range.".to_string())
        }
    }
    fn score(&self) -> f64 {
        self.scores.iter().sum()
    }
    fn profiles(&self) -> Vec<f64> {
        self.slot_machines.iter().map(|x| x.slot.profile())
        .collect::<Vec<f64>>()
    }
}


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
}
