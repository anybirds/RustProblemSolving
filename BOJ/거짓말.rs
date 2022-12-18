// BOJ 1043 거짓말

use std::io::{self, Read};
use std::mem::swap;
use std::str::{FromStr, SplitWhitespace};

const INVALID_OPERATION: &str = "Invalid operation.";
const FAILED_TO_PARSE: &str = "Failed to parse.";

trait Input {
    fn read_one<T>(&mut self) -> T where T: FromStr;
    fn read_two<T, U>(&mut self) -> (T, U) where T: FromStr, U: FromStr;
    fn read_n<T>(&mut self, n: usize) -> Vec<T> where T: FromStr;
}

impl<'a> Input for SplitWhitespace<'a> {
    fn read_one<T>(&mut self) -> T where T: FromStr {
        self.next().unwrap_or_else(|| panic!("{INVALID_OPERATION}"))
        .parse().unwrap_or_else(|_| panic!("{FAILED_TO_PARSE}"))
    }

    fn read_two<T, U>(&mut self) -> (T, U) where T: FromStr, U: FromStr {
        (self.read_one(), self.read_one())
    }

    fn read_n<T>(&mut self, n: usize) -> Vec<T> where T: FromStr {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(self.read_one());
        }
        v
    }
}

struct UnionFind {
    p: Vec<usize>,
    cnt: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut cnt = vec![1; n + 1];
        cnt[0] = 51;
        UnionFind {
            p: (0..(n + 1)).collect(),
            cnt
        }
    }

    fn union(&mut self, u: usize, v: usize) -> usize {
        let mut x = self.find(u);
        let mut y = self.find(v);

        if x != y {
            if self.cnt[x] < self.cnt[y] {
                swap(&mut x, &mut y);
            }
            self.p[y] = x;
            self.cnt[x] += self.cnt[y];
        }
        x
    }

    fn find(&self, mut u: usize) -> usize {
        while u != self.p[u] {
            u = self.p[u];
        }
        u
    }
}

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).unwrap();
    let mut input = string.split_whitespace();
    
    let (n, m) = input.read_two();
    let mut uf = UnionFind::new(n);

    let nt = input.read_one();
    let t = input.read_n(nt);
    for u in t {
        uf.union(0, u);
    }

    let mut q = Vec::new();
    for _ in 0..m {
        let nv = input.read_one();
        let v = input.read_n(nv);
        let mut p = uf.find(v[0]);
        for i in 0..v.len() {
            p = uf.union(p, v[i]);
        }
        q.push(v[0]);
    }

    let mut cnt = 0;
    for u in q {
        cnt += (uf.find(u) != 0) as i32;
    }
    print!("{cnt}");
}