// BOJ 1389 케빈 베이컨의 6단계 법칙

use std::io;
use std::str::FromStr;

fn read_two<T>() -> (T, T) where T: FromStr + Default + Copy {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut arr = [T::default(); 2];
    let mut iter = line.split_whitespace();
    for i in 0..2 {
        arr[i] = match iter.next().unwrap().parse() {
            Ok(t) => t,
            Err(_) => panic!("Failed to parse.")
        }
    }
    (arr[0], arr[1])
}

fn main() {
    let (V, E) = read_two();
    let g = vec![Vec::new(); V];
    for i in 0..E {
        let (u, v) = read_two();
        
    }
}