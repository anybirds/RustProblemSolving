// BOJ 1629 곱셈

use std::io;
use std::str::{FromStr, SplitWhitespace};

const FAILED_TO_PARSE: &str = "Failed to parse.";
const INVALID_OPERATION: &str = "Invalid operation.";

trait Input {
    fn read_one<T>(&mut self) -> T where T: FromStr;
    fn read_three<T, U, V>(&mut self) -> (T, U, V) where T: FromStr, U: FromStr, V: FromStr; 
}

impl<'a> Input for SplitWhitespace<'a> {
    fn read_one<T>(&mut self) -> T where T: FromStr {
        self.next().unwrap_or_else(|| panic!("{INVALID_OPERATION}"))
            .parse().unwrap_or_else(|_| panic!("{FAILED_TO_PARSE}"))
    }

    fn read_three<T, U, V>(&mut self) -> (T, U, V) where T: FromStr, U: FromStr, V: FromStr {
        (self.read_one(), self.read_one(), self.read_one())
    }
}

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    let mut input = string.split_whitespace();
    let (mut a, mut b, c): (usize, usize, usize) = input.read_three();
    let mut ans = 1;
    while b > 0 {
        if b & 1 > 0 {
            ans = (ans * a) % c;   
        }
        a = (a * a) % c;
        b >>= 1;
    }
    print!("{ans}");
}