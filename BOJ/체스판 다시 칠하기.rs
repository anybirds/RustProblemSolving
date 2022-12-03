// BOJ 1018 체스판 다시 칠하기

use std::io;
use std::str::FromStr;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn collect_line<T>() -> Vec<T> where T: FromStr {
    let line = read_line();
    line.split_whitespace().map(
        |split| -> T { 
            match split.parse() { Ok(value) => value, Err(_) => panic!("Failed to parse.") } 
        }
    ).collect()
}

fn read_two_numbers() -> (usize, usize) {
    let integers = collect_line();
    (integers[0], integers[1])
}

const WB: [char; 2] = ['W', 'B'];

fn main() {
    let (n, m) = read_two_numbers();
    let mut retouches = vec![vec![[0, 0]; m + 1]; n + 1];
    for i in 0..n {
        for (j, character) in read_line().trim().chars().enumerate() {
            let indices = [(i + j) % 2, (i + j + 1) % 2];
            if character == WB[indices[0]] {
                retouches[i + 1][j + 1][1] = 1;
            } else {
                retouches[i + 1][j + 1][0] = 1;
            }
        }
    }
    let mut sat = vec![vec![[0, 0]; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            for k in 0..=1 {
                sat[i][j][k] = retouches[i][j][k] + sat[i - 1][j][k] + sat[i][j - 1][k] - sat[i - 1][j - 1][k];
            }
        }
    }
    let mut ans = n * m;
    for i in 8..=n {
        for j in 8..=m {
            for k in 0..=1 {
                ans = ans.min(sat[i][j][k] - sat[i - 8][j][k] - sat[i][j - 8][k] + sat[i - 8][j - 8][k]);
            }
        }
    }
    print!("{}", ans);
}