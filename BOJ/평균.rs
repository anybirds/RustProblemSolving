// BOJ 1546 평균

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line.");
    let n: u32 = line.trim().parse().expect("Failed to parse.");
    
    line.clear();
    io::stdin().read_line(&mut line).expect("Failed to read ilne.");
    let mut m: u32 = 0;
    let mut sum: u32 = 0;
    for split in line.split_whitespace() {
        let score = split.parse().expect("Failed to parse.");
        sum += score;
        m = m.max(score);
    }
    print!("{}", ((sum as f64 / n as f64) / m as f64) * 100.0);
}