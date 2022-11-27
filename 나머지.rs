// BOJ 3052 나머지

use std::io;
use std::collections::HashSet;

fn main() {
    let mut hash_set = HashSet::new();
    for _ in 0..10 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line.");
        let n: u32 = line.trim().parse().expect("Failed to parse.");
        hash_set.insert(n % 42);
    }
    print!("{}", hash_set.len());
}