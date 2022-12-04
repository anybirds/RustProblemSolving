// BOJ 2609 최대공약수와 최소공배수

use std::io;
use std::mem::swap;
use std::str::FromStr;

fn read_two<T>() -> (T, T) where T: FromStr + Default + Copy {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut arr = [T::default(); 2];
    for (i, split) in line.split_whitespace().enumerate() {
        arr[i] = match split.parse() {
            Ok(t) => t,
            Err(_) => panic!("Failed to parse.")
        }
    }
    (arr[0], arr[1])
}

fn gcd(mut n: u32, mut m: u32) -> u32 {
    loop {
        if n > m {
            swap(&mut n, &mut m);
        }
        if n == 0 {
            return m;
        }
        m -= (m / n) * n;
    }
}

fn lcm(n: u32, m: u32) -> u32 {
    let g = gcd(n, m);
    g * (n / g) * (m / g)
}

fn main() {
    let (n, m) = read_two();
    println!("{}", gcd(n, m));
    println!("{}", lcm(n, m));
}