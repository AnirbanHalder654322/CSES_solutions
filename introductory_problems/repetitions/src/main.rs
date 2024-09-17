use std::io::stdin;
use std::io::{self, Stdin};
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

    let mut str = scan.token::<String>().into_bytes().into_iter();
    let mut count: usize = 1;
    let mut max_count: usize = 1;

    let mut temp = str.next().unwrap();

    for s in str {
        if s == temp {
            count += 1;
        } else {
            count = 1;
            temp = s;
        }
        max_count = std::cmp::max(count, max_count);
    }
    println!("{:?}", max_count);
}
