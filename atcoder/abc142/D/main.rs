use io::*;
use std::*;
use std::collections::HashSet;


fn calc_factors(x: i64) -> HashSet<i64> {
    let mut y = x;
    let sq = (x as f32).sqrt().ceil() as i64;
    let mut factors: HashSet<i64> = HashSet::new();
    for n in 2..sq {
        while y % n == 0 {
            factors.insert(n);
            y /= n;
        } 
    }
    if y > 1 {
        factors.insert(y);
    }
    return factors
}

fn solve(A: i64, B: i64) {
    let a = calc_factors(A);
    let b = calc_factors(B);
    let ans = a.intersection(&b).collect::<HashSet<&i64>>().len() + 1;
    println!("{}", ans);
}

// Generated by 1.1.7.1 https://github.com/kyuridenamida/atcoder-tools  (tips: You use the default template now. You can remove this line by using your custom template)
fn main() {
    let con = read_string();
    let mut scanner = Scanner::new(&con);
    let mut A: i64;
    A = scanner.next();
    let mut B: i64;
    B = scanner.next();
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(move || solve(A, B)).unwrap().join().unwrap();
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
