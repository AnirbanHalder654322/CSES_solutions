use std::io::stdin;
use std::io::{self, Stdin};
use std::{i64, str};

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

    let n = scanner.token::<usize>();

    let m = scanner.token::<usize>();

    let k = scanner.token::<usize>();

    let mut a = vec![];

    for i in 0..n {
        a.push(scanner.token::<usize>());
    }
    let mut b = vec![];

    for i in 0..m {
        b.push(scanner.token::<usize>());
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut left_ptr = 0 as i64;
    let mut right_ptr = 0 as i64;
    let mut count: usize = 0;

    while left_ptr < n as i64 && right_ptr < m as i64 {
        if a[left_ptr as usize].abs_diff(b[right_ptr as usize]) <= k {
            count += 1;
            left_ptr += 1;
            right_ptr += 1;
        } else if a[left_ptr as usize] < b[right_ptr as usize] {
            left_ptr += 1;
        } else {
            right_ptr += 1;
        }
    }
    println!("{}", count);
}
