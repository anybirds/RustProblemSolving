// BOJ 18870 좌표 압축

use std::io;
use std::str::FromStr;

const FAILED_TO_PARSE: &str = "Failed to parse.";

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap_or_else(|_| panic!("{FAILED_TO_PARSE}"))
}

fn read_vec<T>() -> Vec<T> where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.split_whitespace().map(|x| x.parse().unwrap_or_else(|_| panic!("{FAILED_TO_PARSE}"))).collect()
}

fn main() {
    let n: usize = read_one();
    let x: Vec<i32> = read_vec();
    
    assert_eq!(n, x.len());

    let mut v: Vec<(usize, i32)> = x.into_iter().enumerate().collect();
    
    v.sort_unstable_by(|(_, l), (_, r)| l.cmp(r));
    
    let mut ans = vec![0; n];
    let mut cnt = 0;
    let mut p = v[0].1;
    for (i, x) in v {
        if p < x {
            cnt += 1;         
        }
        ans[i] = cnt;
        p = x;
    }
    for i in ans {
        print!("{} ", i);
    }
}