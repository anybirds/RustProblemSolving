// BOJ 11050 이항 계수

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line.");
    let mut split = line.split_whitespace();
    let n: usize = split.next().unwrap().parse().expect("Failed to parse.");
    let mut k: usize = split.next().unwrap().parse().expect("Failed to parse.");
    
    k = k.min(n - k);
    let mut coeffs = vec![vec![0; k + 1]; n + 1];
    for i in 0..=n {
        coeffs[i][0] = 1;
    }
    for i in 1..=n {
        for j in 1..=k {
            coeffs[i][j] = coeffs[i - 1][j] + coeffs[i - 1][j - 1];
        }
    }
    print!("{}", coeffs[n][k]);
}