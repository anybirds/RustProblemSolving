// BOJ 1654 랜선 자르기 

use std::io;
use std::str::FromStr;

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.trim().parse() { Ok(i) => i, Err(_) => panic!("Failed to parse.") }
}

fn read_vec<T>() -> Vec<T> where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap(); 
    line.split_whitespace().map(|x| -> T { 
        match x.parse() { Ok(i) => i, Err(_) => panic!("Failed to parse.") }
    }).collect()
}

fn read_two<T>() -> (T, T) where T: FromStr + Copy {
    let integers = read_vec();
    (integers[0], integers[1])
}

fn binary_search(lengths: &Vec<u32>, n: usize, l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    let m = (l + r) / 2 + 1;
    let mut count = 0;
    for length in lengths {
        count += length / m;
    }
    if count >= n as u32 {
        return binary_search(lengths, n, m, r);
    } else {
        return binary_search(lengths, n, l, m - 1);
    }
}

fn main() {
    let (k, n) = read_two();
    let mut lengths = vec![0_u32; k];
    for i in 0..k {
        lengths[i] = read_one();
    }
    print!("{}", binary_search(&lengths, n, 0, i32::MAX as u32));
}