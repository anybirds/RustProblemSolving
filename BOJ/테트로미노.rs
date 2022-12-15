// BOJ 14500 테트로미노

use std::io;
use std::str::{FromStr, SplitWhitespace};

const FAILED_TO_PARSE: &str = "Failed to parse.";
const INVALID_FORMAT: &str = "Invalid format.";

fn read_iter<T>(iter: &mut SplitWhitespace) -> T where T: FromStr {
    iter.next()
        .unwrap_or_else(|| panic!("{}", INVALID_FORMAT))
        .parse()
        .unwrap_or_else(|_| panic!("{}", FAILED_TO_PARSE))
}

fn read_two<T, U>() -> (T, U) where T: FromStr, U: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    (read_iter(&mut iter), read_iter(&mut iter))
}

fn read_vec<T>() -> Vec<T> where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|x| x.parse().unwrap_or_else(|_| panic!("{}", FAILED_TO_PARSE)))
        .collect()
}

const TETROMINO: [[(i32, i32); 3]; 19] = [
    [(0, 1), (0, 2), (0, 3)],
    [(1, 0), (2, 0), (3, 0)],

    [(0, 1), (1, 0), (1, 1)],

    [(1, 0), (2, 0), (2, 1)],
    [(1, 0), (2, 0), (2, -1)],
    [(0, -1), (0, -2), (1, -2)],
    [(0, -1), (0, -2), (-1, -2)],
    [(-1, 0), (-2, 0), (-2, -1)],
    [(-1, 0), (-2, 0), (-2, 1)],
    [(0, 1), (0, 2), (-1, 2)],
    [(0, 1), (0, 2), (1, 2)],

    [(1, 0), (1, 1), (2, 1)],
    [(0, -1), (1, -1), (1, -2)],
    [(1, 0), (1, -1), (2, -1)],
    [(0, 1), (1, 1), (1, 2)],

    [(0, 1), (0, 2), (1, 1)],
    [(1, 0), (2, 0), (1, -1)],
    [(0, 1), (0, 2), (-1, 1)],
    [(1, 0), (2, 0), (1, 1)]
];

fn is_valid(n: usize, m: usize, y: i32, x: i32) -> bool {
    y >= 0 && y < n as i32 && x >= 0 && x < m as i32
}

fn main() {
    let (n, m) = read_two();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(read_vec())
    }
    let mut max = 0;
    for y in 0..n {
        for x in 0..m {
            for t in TETROMINO {
                let mut sum: i32 = v[y][x];
                for (dy, dx) in t {
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;
                    if !is_valid(n, m, ny, nx) {
                        break;
                    }
                    sum += v[ny as usize][nx as usize];
                }
                max = max.max(sum);
            }
        }
    }
    print!("{}", max);
}
