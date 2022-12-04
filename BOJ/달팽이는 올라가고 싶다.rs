// BOJ 2869 달팽이는 올라가고 싶다

use std::io;
use std::str::FromStr;

fn read_three<T>() -> (T, T, T) where T: FromStr + Default + Copy {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut arr = [T::default(); 3];
    for (i, split) in line.split_whitespace().enumerate() {
        arr[i] = match split.parse() {
            Ok(t) => t,
            Err(_) => panic!("Failed to parse.")
        }
    }
    (arr[0], arr[1], arr[2])
}

fn main() {
    let (a, b, v) = read_three::<i32>();
    print!("{}", (v - b - 1) / (a - b) + 1);
}