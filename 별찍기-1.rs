// BOJ 2438 별찍기-1

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line.");
    let n: u32 = line.trim().parse().expect("Failed to parse.");
    for i in 1..=n {
        for _ in 0..i {
            print!("*");
        }
        print!("\n");
    }
}
