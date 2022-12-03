// BOJ 2439 별찍기-2

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to readl line.");
    let n: u32 = line.trim().parse().expect("Failed to parse.");
    
    for i in 1..=n {
        for _ in (0..n-i).rev() {
            print!(" ");
        }
        for _ in 1..=i {
            print!("*");
        }
        print!("\n");
    }
}