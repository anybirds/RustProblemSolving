// BOJ 2164 카드2

use std::io;
use std::str::FromStr;
use std::collections::VecDeque;

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.trim().parse() {
        Ok(t) => t,
        Err(_) => panic!("Failed to parse.")
    }
}
fn main() {
    let n = read_one();
    let mut v = VecDeque::new();
    for i in 1..=n {
        v.push_back(i);
    }
    loop {
        if v.len() == 1 {
            break;
        }
        v.pop_front();
        let item = v.pop_front().unwrap();
        v.push_back(item);
    }
    print!("{}", v[0]);
}