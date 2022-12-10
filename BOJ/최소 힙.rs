// BOJ 1927 최소 힙

use std::io;
use std::io::Write;
use std::str::FromStr;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.trim().parse() {
        Ok(t) => t,
        Err(_) => panic!("Failed to parse.")
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let n = read_one();
    let mut h = BinaryHeap::new();
    for _ in 0..n {
        let x: i32 = read_one();
        if x == 0 {
            match h.pop() {
                Some(Reverse(v)) => { writeln!(out, "{}", v).unwrap() }
                None => { writeln!(out, "0").unwrap() }
            }
        } else {
            h.push(Reverse(x));
        }
    }
}