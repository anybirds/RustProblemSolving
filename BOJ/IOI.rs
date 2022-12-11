// BOJ 5525 IOIOI

use std::io;
use std::str::FromStr;

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap_or_else(|_| { panic!("Failed to parse."); })
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn main() {
    let n: i32 = read_one();
    let m: usize = read_one();
    let s = read_line();
    
    assert_eq!(m, s.trim().len());

    let mut x = 0;
    let mut y;
    let f = [[1, 0], [1, 2], [1, 0]];
    let mut cnt = 0;
    let mut sum = 0;
    for c in s.chars() {
        match c {
            'I' => {
                y = f[x][0];
            }
            'O' => {
                y = f[x][1];
            }
            _ => {
                sum += (cnt - n + 1).max(0);
                break;
            }
        }
        if y == 0 || (x == 1 && y == 1) {
            sum += (cnt - n + 1).max(0);
            cnt = 0;
        }
        if x == 2 && y == 1 {
            cnt += 1;
        }
        x = y;
    }
    print!("{}", sum);
}