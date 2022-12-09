// BOJ 1992 쿼드트리

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

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn quad_tree(s: &mut String, m: &Vec<Vec<char>>, y: usize, x: usize, w: usize) {
    if w <= 1 {
        s.push(m[y][x]);
        return;
    }

    s.push('(');
    
    let h = w >> 1;
    quad_tree(s, m, y, x, h);
    quad_tree(s, m, y, x + h, h);
    quad_tree(s, m, y + h, x, h);
    quad_tree(s, m, y + h, x + h, h);
    
    let p = &s[s.len() - 4..];
    if p == "0000" {
        for _ in 0..5 {
            s.pop();
        }
        s.push('0');
    } else if p == "1111" {
        for _ in 0..5 {
            s.pop();
        }
        s.push('1');
    } else {
        s.push(')');
    }
}

fn main() {
    let n = read_one();
    let mut m = vec![vec!['0'; n]; n];
    for y in 0..n {
        let line = read_line();
        for (x, c) in line.char_indices() {
            m[y][x] = c;
        }
    }
    let mut s = String::new();
    quad_tree(&mut s, &m, 0, 0, n);
    print!("{}", s);
}