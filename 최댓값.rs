// BOJ 2562 최댓값

use std::io;

fn main() {
    let mut m = 0;
    let mut index = 0;
    for i in 1..=9 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line.");
        let n: i32 = line.trim().parse().expect("Failed to parse line.");
        if m < n {
            index = i;
            m = n;
        }
    }
    println!("{}", m);
    println!("{}", index);
}