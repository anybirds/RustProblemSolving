// BOJ 9019 DSLR

use std::io;
use std::str::{FromStr, SplitWhitespace};
use std::collections::VecDeque;
use std::io::Write;

const FAILED_TO_PARSE: &str = "Failed to parse.";
const INVALID_FORMAT: &str = "Invalid format.";

const DSLR: [(char, fn(usize) -> usize); 4] = [
    ('D', |x| (x * 2) % 10000),
    ('S', |x| (x + 9999) % 10000),
    ('L', |x| (x % 1000) * 10 + x / 1000),
    ('R', |x| (x % 10) * 1000 + (x / 10))
    ];

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap_or_else(|_| panic!("{}", FAILED_TO_PARSE))
}

fn read_iter<T>(iter: &mut SplitWhitespace) -> T where T: FromStr {
    iter.next()
        .unwrap_or_else(|| panic!("{}", INVALID_FORMAT))
        .parse()
        .unwrap_or_else(|_| panic!("{}", FAILED_TO_PARSE))
}

fn read_two<T, U>() -> (T, U) where T: FromStr, U: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    (read_iter(&mut iter), read_iter(&mut iter)) 
}

fn bfs(a: usize, b: usize) -> String {
    let mut q = VecDeque::new();
    let mut p = vec![(usize::default(), char::default()); 10000];
    let mut visit = [false; 10000];

    q.push_back(a);
    visit[a] = true;
    p[a] = (a, char::default());
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for (c, f) in DSLR {
            let v = f(u);
            if !visit[v] {
                q.push_back(v);
                visit[v] = true;
                p[v] = (u, c);
            }
        }
        if visit[b] {
            break;
        }
    }
    
    let mut s = Vec::new();
    let mut v = b;
    while v != a {
        let (u, c) = p[v];
        s.push(c);
        v = u;
    }
    s.into_iter().rev().collect()
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let t = read_one();
    for _ in 0..t {
        let (a, b) = read_two();
        writeln!(out, "{}", bfs(a, b)).unwrap();
    }
}