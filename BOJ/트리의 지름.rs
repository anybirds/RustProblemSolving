// BOJ 1167 트리의 지름

use std::io::{self, Read};
use std::str::{FromStr, SplitWhitespace};

const FAILED_TO_PARSE: &str = "Failed to parse.";
const INVALID_OPERATION: &str = "Invalid operation.";

trait Input {
    fn read_one<T>(&mut self) -> T where T: FromStr;
}

impl<'a> Input for SplitWhitespace<'a> {
    fn read_one<T>(&mut self) -> T where T: FromStr {
        self.next().unwrap_or_else(|| panic!("{INVALID_OPERATION}"))
            .parse().unwrap_or_else(|_| panic!("{FAILED_TO_PARSE}"))
    }
}

fn dfs(t: &Vec<Vec<(usize, usize)>>, visit: &mut Vec<bool>, u: usize) -> (usize, usize) {
    visit[u] = true;
    let mut first = 0;
    let mut second = 0;
    let mut diameter = 0;
    for &(v, w) in &t[u] {
        if !visit[v] {
            let sub = dfs(t, visit, v);
            diameter = diameter.max(sub.0);
            let height = sub.1 + w;
            if height > first {
                second = first;
                first = height;
            } else if height > second { 
                second = height;
            }
        }
    }
    (diameter.max(first + second), first)
} 

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).unwrap();
    let mut input = string.split_whitespace();

    let n = input.read_one();
    let mut t = vec![vec![]; n];
    for _ in 0..n {
        let mut u: usize = input.read_one();
        u -= 1;
        loop {
            let mut v: i32 = input.read_one();
            if v == -1 {
                break;
            }
            v -= 1;
            let w: usize = input.read_one();
            t[u].push((v as usize, w));
        }
    }
    
    let mut visit = vec![false; n];
    print!("{}", dfs(&t, &mut visit, 0).0);
}