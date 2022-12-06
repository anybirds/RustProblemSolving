// BOJ 1107 리모컨

use std::io;
use std::str::FromStr;
use TestResult::*;

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
    line.split_whitespace().map(|x| -> T {
        match x.parse() {
            Ok(t) => t,
            Err(_) => panic!("Failed to parse.")
        }
    }).collect()
}

enum TestResult {
    Success(i32),
    Fail,
}

fn test_pressable(v: &Vec<i32>, mut i: i32) -> TestResult {
    let mut c = 0;
    loop {
        let d = i % 10;
        let mut p = true;
        for b in v {
           if *b == d {
                p = false;
           }
        }
        if p {
            i /= 10;
            c += 1;
        } else {
            return Fail;
        }

        if i <= 0 { break; }  
    }
    
    Success(c)
}

fn main() {
    let n: i32 = read_one();
    let m = read_one();
    let v;
    match m {
        0 => { v = Vec::new(); } 
        _ => { v = read_vec(); }
    }
    
    assert_eq!(v.len(), m);

    let mut min = i32::MAX;
    for i in 0..1000000 {
        match test_pressable(&v, i) {
            Success(c) => {
                min = min.min((n - i).abs() + c);
            }
            Fail => continue
        }
    }
    min = min.min((n - 100).abs());
    print!("{}", min);
}