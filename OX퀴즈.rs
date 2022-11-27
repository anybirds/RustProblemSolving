// BOJ 8958 OX퀴즈

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line.");
    let n: u32 = line.trim().parse().expect("Failed to parse.");

    for _ in 0..n {
        line.clear();
        io::stdin().read_line(&mut line).expect("Failed to read line.");
        
        let mut score = 0;
        let mut sum = 0;
        for character in line.trim().chars() {
            if character == 'O' {
                score += 1;
                sum += score;
            } else if character == 'X' {
                score = 0;
            }
        }

        println!("{}", sum);
    }
}