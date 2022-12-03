// BOJ 1152 단어의 개수

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line.");
    let line = line.trim();
    print!("{}", line.split_whitespace().count());
}