// BOJ 1389 케빈 베이컨의 6단계 법칙

use std::io;
use std::str::FromStr;
use std::collections::VecDeque;

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

fn bfs(g: &Vec<Vec<usize>>, s: usize) -> i32 {
    let mut dist = vec![-1; g.len()];
    let mut q = VecDeque::new();
    q.push_back(s);
    dist[s] = 0;
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for &v in &g[u] {
            if dist[v] < 0 {
                q.push_back(v);
                dist[v] = dist[u] + 1;
            }
        }
    }
    let mut sum = 0;
    for d in dist {
        sum += d;
    }
    sum
}

fn main() {
    let (v, e) = read_two();
    let mut g = vec![Vec::new(); v];
    for _ in 0..e {
        let (mut x, mut y): (usize, usize) = read_two();
        x -= 1;
        y -= 1;
        g[x].push(y);
        g[y].push(x);
    }
    let mut min = i32::MAX;
    let mut index = 0;
    for i in 0..v {
        let count = bfs(&g, i);
        if count < min {
            min = count;
            index = i;
        }
    }
    print!("{}", index + 1);
}