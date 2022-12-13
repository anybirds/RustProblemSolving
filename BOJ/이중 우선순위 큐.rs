// BOJ 7662 이중 우선순위 큐

use std::io;
use std::str::{FromStr, SplitWhitespace};
use std::collections::BTreeMap;

const FAILED_TO_PARSE: &str = "Failed to parse.";
const INVALID_INSTRUCTION: &str = "Invalid instruction.";

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap_or_else(|_| panic!("{}", FAILED_TO_PARSE))
}

fn read_iter<T>(iter: &mut SplitWhitespace) -> T where T: FromStr { 
    iter.next().unwrap().parse().unwrap_or_else(|_| panic!("{}", FAILED_TO_PARSE))
}

fn read_two<T, U>() -> (T, U) where T: FromStr, U: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    (read_iter(&mut iter), read_iter(&mut iter))
}

fn main() {
    let t = read_one();
    for _ in 0..t {
        let mut m = BTreeMap::new();
        let k = read_one();
        for _ in 0..k {
            let (i, n): (char, i32) = read_two();
            match i {
                'I' => {
                    let v = m.get(&n).unwrap_or_else(|| &0);
                    m.insert(n, *v + 1);
                }
                'D' => {
                    if !m.is_empty() {
                        if n == -1 {
                            if let Some((&k, &v)) = m.iter().next() {
                                if v > 1 {
                                    m.insert(k, v - 1);
                                } else {
                                    m.remove(&k);
                                }
                            }
                        } else if n == 1 {
                            if let Some((&k, &v)) = m.iter().rev().next() {
                                if v > 1 {
                                    m.insert(k, v - 1);
                                } else {
                                    m.remove(&k);
                                }
                            }
                        }
                    }
                }
                _ => panic!("{}", INVALID_INSTRUCTION)
            }
        }   
        if m.is_empty() {
            println!("EMPTY");
        } else {
            println!("{} {}", m.iter().rev().next().unwrap().0, m.iter().next().unwrap().0);
        }
    }
}