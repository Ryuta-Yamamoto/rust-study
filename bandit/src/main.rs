use std::vec::Vec;
use statrs::distribution::Beta;
use rand::distributions::Distribution;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, thread_rng};


struct SlotMachine<T: Rng> {
    prob: f64,
    rng: T,
    rewards: Vec<f64>
}

impl<StdRng: SeedableRng + Rng> SlotMachine<StdRng> {
    fn new(num: usize) -> SlotMachine<StdRng> {
        SlotMachine {
            prob: rand::random(),
            rng: StdRng::seed_from_u64(num as u64),
            rewards: vec![],
        }
    }
    fn play(&mut self) -> f64 {
        let reward: f64 = (self.rng.gen::<f64>() < self.prob) as u32 as f64;
        self.rewards.push(reward);
        reward
    }
    fn get_prob(&self) -> f64 {
        self.prob
    }
    fn get_rewards(&self) -> &Vec<f64> {
        &self.rewards
    }
}


enum Error {
    IndexError(String),
    MaxTrialError,
}


struct Casino {
    slots: Vec<SlotMachine<StdRng>>,
    selections: Vec<usize>,
    rewards: Vec<f64>,
    max_trial: usize
}


impl Casino {
    fn new(n_slots: usize, max_trial: usize) -> Casino {
        let mut slots: Vec<SlotMachine<StdRng>> = vec![];
        for n in 0..n_slots {
            slots.push(SlotMachine::new(n));
        }
        Casino {
            slots,
            selections: vec![],
            rewards: vec![],
            max_trial,
        }
    }
    fn play(&mut self, idx: usize) -> Result<f64, Error> {
        if idx >= self.slots.len() {
            return Err(Error::IndexError("Index out of range".to_string()))
        }
        if self.rewards.len() >= self.max_trial {
            return Err(Error::MaxTrialError)
        }
        let val = self.slots[idx].play();
        self.selections.push(idx);
        self.rewards.push(val);
        Result::Ok(val)
    }
    fn play_verbose(&mut self, idx: usize) -> Result<f64, Error> {
        let res = self.play(idx);
        match &res {
            Ok(val) => {
                println!("play slot {}, got reward {}", idx, val);
            }
            Err(Error::IndexError(val)) => println!("Error ocurred: {}", val),
            Err(Error::MaxTrialError) => println!("You can't play anymore")
        }
        res
    }
}


fn mean(v: &Vec<f64>) -> Option<f64> {
    if v.is_empty() {return None}
    let mut total: f64 = 0.;
    for val in v.iter() {
        total += val;
    }
    Some(total / v.len() as f64)
}


fn argmax<T: std::cmp::PartialOrd>(v: &Vec<T>) -> Option<usize> {
    if v.is_empty() {return None}
    let mut max_idx: usize = 0;
    let mut max_val = v.iter().next()?;
    for (i, val) in v.iter().enumerate() {
        if val > max_val {
            max_idx = i;
            max_val = val;
        }
    }
    Some(max_idx)
}


trait Algorithm {
    fn select(&self) -> usize;
    fn obtain(&mut self, reward: f64, idx: usize);
}

struct RandomPolicy {n_arms: usize}

impl RandomPolicy {fn new(n_arms: usize) -> RandomPolicy {RandomPolicy{n_arms}}}

impl Algorithm for RandomPolicy {
    fn select(&self) -> usize {
        rand::random::<usize>() % self.n_arms
    }
    fn obtain(&mut self, reward: f64, idx: usize) {}
}

struct EpsilonGreedyPolicy {
    epsilon: f64,
    histories: Vec<Vec<f64>>
}

impl EpsilonGreedyPolicy {
    fn new(epsilon: f64, n_slots: usize) -> Result<EpsilonGreedyPolicy, String> {
        if epsilon > 1.0 || epsilon < 0. {
            return Err("epsilon must be between 0 and 1".to_string())
        }
        Ok(EpsilonGreedyPolicy {
            epsilon,    
            histories: vec![vec![]; n_slots]
        })
    }
}

impl Algorithm for EpsilonGreedyPolicy {
    fn select(&self) -> usize {
        if rand::random::<f64>() < self.epsilon{
            return rand::random::<usize>() % self.histories.len()
        }
        let means: Vec<f64> = self.histories.iter().map(|x| mean(x).unwrap_or_else(|| 1.)).collect();
        argmax(&means).unwrap()
    }
    fn obtain(&mut self, reward: f64, idx: usize) {
        self.histories[idx].push(reward);
    }
}


struct TSPolicy {
    alpha: f64,
    beta: f64,
    histories: Vec<Vec<f64>>
}

impl TSPolicy {
    fn new(alpha: f64, beta: f64, n_slots: usize) -> Result<TSPolicy, String> {
        if alpha <= 0. || beta <= 0. {
            return Err("alpha and beta must be >0".to_string())
        }
        Ok(TSPolicy{
            alpha, 
            beta,
            histories: vec![vec![]; n_slots]
        })
    }
}


impl Algorithm for TSPolicy {
    fn select(&self) -> usize {
        let samples = self.histories.iter().map(
            |v| {
                let a = v.iter().sum::<f64>() + &self.alpha;
                let b = v.len() as f64 - v.iter().sum::<f64>() + &self.beta;
                Beta::new(a, b).unwrap().sample(&mut thread_rng())
            }
        ).collect();
        argmax(&samples).unwrap()
    }
    fn obtain(&mut self, reward: f64, idx: usize) {
        self.histories[idx].push(reward);
    }
}

fn play_and_obrain<T: Algorithm>(casino: &mut Casino, policy: &mut T) {
    let idx = policy.select();
    if let Ok(reward) = casino.play(idx) {
        policy.obtain(reward, idx);
    } else {
        panic!();
    }
}


fn main() {
    let n_slots: usize = 100;
    let max_trial = 10000;
    let mut casino = Casino::new(n_slots, max_trial * 3);
    let mut rp = RandomPolicy::new(n_slots);
    let mut ts = TSPolicy::new(1., 1., n_slots).unwrap();
    let mut eg = EpsilonGreedyPolicy::new(0.05, n_slots).unwrap();

    for _ in 0..max_trial {
        play_and_obrain(&mut casino, &mut rp);
        play_and_obrain(&mut casino, &mut ts);
        play_and_obrain(&mut casino, &mut eg);
    }
    let mut sum: f64 = 0.;
    for f in ts.histories.iter().flatten() {
        sum += f;
    }
    println!("ts {:?}", sum);
    sum = 0.;
    for f in eg.histories.iter().flatten() {
        sum += f;
    }
    println!("eg {:?}", sum);
}
