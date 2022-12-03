// BOJ 1966 프린터 큐

use std::io;
use std::str::FromStr;

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    match line.trim().parse() {
        Ok(t) => t,
        Err(_) => panic!("Failed to parse.")
    }
}

fn read_vec<T>() -> Vec<T> where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(
        |x| -> T {
            match x.parse() {
                Ok(t) => t,
                Err(_) => panic!("Failed to parse.")
            }
        }
    ).collect()
}

fn read_two<T>() -> (T, T) where T: FromStr + Copy {
    let vec = read_vec();
    (vec[0], vec[1])
}

fn main() {
    let t = read_one();
    for _ in 0..t {
        let (n, m) = read_two();
        let v: Vec<i32> = read_vec();
        let mut s = v.clone();
        s.sort_unstable_by(|l, r| r.cmp(l) );
        // println!("{:?}", s);
        
        let mut visit = vec![false; n];
        let mut count = 0;
        let mut i = 0;
        loop {
            if i >= v.len() {
                i = 0;
            }
            
            if !visit[i] {
                if v[i] == s[count] {
                    visit[i] = true;
                    count += 1;
                    if i == m {
                        break;
                    }
                }
            }
            i += 1;
        }

        println!("{}", count);
    }
}