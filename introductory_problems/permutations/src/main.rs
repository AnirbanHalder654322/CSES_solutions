use std::io::{self};
use std::process::exit;
use std::str;

/// Reads white-space separated tokens one at a time.
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    /// Use "turbofish" syntax token::<T>() to select data type of next token.
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    solve();
}

fn solve() {
    let mut scan = Scanner::new(io::stdin().lock());

    let n = scan.token::<usize>();

    if n == 1 {
        print!("1");
        exit(0);
    }

    if n <= 3 {
        print!("NO SOLUTION");

        exit(0);
    }

    for i in 1..=n {
        if i % 2 == 0 {
            print!("{} ", i);
        }
    }
    for i in 1..=n {
        if i % 2 != 0 {
            print!("{} ", i);
        }
    }
}
