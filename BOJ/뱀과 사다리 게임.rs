// BOJ 16928 뱀과 사다리 게임

use std::io;
use std::str::{FromStr, SplitWhitespace};
use std::collections::{HashMap, VecDeque};

const FAILED_TO_PARSE: &str = "Failed to parse.";
const INVALID_OPERATION: &str = "Invalid operation.";

fn read_iter<T>(iter: &mut SplitWhitespace) -> T where T: FromStr {
    iter.next().unwrap_or_else(|| panic!("{INVALID_OPERATION}"))
        .parse().unwrap_or_else(|_| panic!("{FAILED_TO_PARSE}"))
}

fn read_two<T, U>() -> (T, U) where T: FromStr, U: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    (read_iter(&mut iter), (read_iter(&mut iter)))
}

fn main() {
    let (n, m): (usize, usize) = read_two();
    let mut h = HashMap::<usize, usize>::new();
    for _ in 0..(n + m) {
        let (u, v) = read_two();
        h.insert(u, v);
    }
    let mut q = VecDeque::new();
    let mut dist = vec![0; 101];
    q.push_back(1_usize);
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        if u == 100 {
            break;
        }
        for i in 1_usize..=6 {
            let mut v = u + i;
            loop {
                if v <= 100 && dist[v] == 0 {
                    dist[v] = dist[u] + 1;
                } else {
                    break;
                }
                if let Some(&w) = h.get(&v) {
                    v = w;
                } else {
                    q.push_back(v);
                    break;
                }
            }
        }
    }
    print!("{}", dist[100]);
}