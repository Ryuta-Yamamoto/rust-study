use io::*;
use std::*;

fn flip_lr(chars: Vec<char>) -> Vec<char> {
    let mut chars = chars.iter().map(|x| if x == &'L'{ 'R' } else { 'L' }).collect();
    chars
}


fn solve(N: i64, K: i64, S: String) {
    let mut chars = S.chars().collect::<Vec<char>>();
    let mut idx = chars.len();
    if chars.len() == 1 { println!("0"); return }
    if chars[0] == 'L' {
        chars = flip_lr(chars);
    }
    for i in 1..chars.len() {
        if chars[i - 1..i + 1] == ['R', 'L'] { idx = i; break; }
    }
    for _ in 0..K {
        if chars.get(idx) == None {
            while chars.get(idx) == Some(&'L') {chars[idx] = 'R'; idx += 1; }
            break;
        }
        while chars.get(idx) == Some(&'L') { chars[idx] = 'R'; idx += 1; }
        while chars.get(idx) == Some(&'R') { idx += 1; }
    }
    let mut score = 0;
    for i in 1..chars.len() {
        if let Some(&[x, y]) = chars.get((i - 1)..(i + 1)) {
            if x == y { score += 1; }
        }
    }
    println!("{}", score);
}

// Generated by 1.1.7.1 https://github.com/kyuridenamida/atcoder-tools  (tips: You use the default template now. You can remove this line by using your custom template)
fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut N: i64;
    N = scanner.next();
    let mut K: i64;
    K = scanner.next();
    let mut S: String;
    S = scanner.next();
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(N, K, S)).unwrap().join().unwrap();
}

pub mod io {
    use std;
    use std::str::FromStr;

    pub struct Scanner<'a> {
        iter: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let s = self.iter.next().unwrap();
            if let Ok(v) = s.parse::<T>() {
                v
            } else {
                panic!("Parse error")
            }
        }

        pub fn next_vec_len<T: FromStr>(&mut self) -> Vec<T> {
            let n: usize = self.next();
            self.next_vec(n)
        }

        pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next()).collect()
        }
    }

    pub fn read_string() -> String {
        use std::io::Read;

        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }

    pub fn read_line() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    }
}
