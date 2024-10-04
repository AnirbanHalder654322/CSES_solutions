use std::collections::HashSet;
use std::io::stdin;
use std::io::{self};
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
    let mut scanner = Scanner::new(stdin().lock());

    let t = scanner.token::<usize>();
    let mut ele: HashSet<usize> = std::collections::HashSet::new();
    for _ in 0..t {
        ele.insert(scanner.token::<usize>());
    }

    println!("{}", ele.len());
}
