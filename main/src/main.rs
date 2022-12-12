// BOJ 7662 이중 우선순위 큐

/*

dual priority queue

get final result!

offline query i guess

no, binary tree, or BTSet can solve this

how about solving this by offline query?

how to impl double ended pq?

order of query matters, but guess it's possible to solve by offline query

sort the entire values given, give it's order by index

removing min does not interrupt max as long as it doesn't make it empty

count pop operation and if it's less than amount of insertion, just do more ..

*/

use std::io;
use std::str::{FromStr, SplitWhitespace};
use std::collections::BinaryHeap;
use std::iter::Rev;

const FAILED_TO_PARSE: &str = "Failed to parse.";
const INVALID_INSTRUCTION: &str = "Invalid instruction.";

fn read_one<T>() -> T where T: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap_or_else(|e| panic!("{}", FAILED_TO_PARSE))
}

fn read_iter<T>(iter: &mut SplitWhitespace) -> T where T: FromStr { 
    iter.next().unwrap().parse().unwrap_or_else(|e| panic!("{}", FAILED_TO_PARSE))
}

fn read_two<T, U>() -> (T, U) where T: FromStr, U: FromStr {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    (read_iter(&mut iter), read_iter(&mut iter))
}

#[derive(Copy, Clone)]
enum HeapType {
    Max = 1,
    Min = -1,
}

enum HeapValue<T> {
    Max(T),
    Min(Rev<T>)
}

fn is_empty(v: &Vec<(char, i32)>) -> bool {
    let mut i_cnt = 0;
    let mut d_cnt = 0;
    for &(i, n) in v {
        match i {
            'I' => i_cnt += 1,
            'D' => d_cnt += 1,
            _ => panic!("{}", INVALID_INSTRUCTION)
        }
    }
    i_cnt <= d_cnt
}

/*
need to make this work for both Min and Max
*/
fn run(v: &Vec<(char, i32)>, t: HeapType) -> i32 {
    let mut h = BinaryHeap::new();
    let mut cnt = 0;
    for &(i, n) in v {
        match i {
            'I' => {
                h.push(n);
            }
            'D' => {
                if cnt < h.len() && n == t as i32 {
                    h.pop();
                }
                cnt += 1;
            }
            _ => panic!("{}", INVALID_INSTRUCTION)
        }
    }
    h.pop().unwrap()
}

fn main() {
    let t = read_one();
    for _ in 0..t {
        let k = read_one();
        let mut v = Vec::new();
        for _ in 0..k {
            let (i, n) = read_two();
            v.push((i, n));
        }
        
        if is_empty(&v) {
            println!("EMPTY");
        } else {
            println!("{} {}", run(&v, HeapType::Max), run(&v, HeapType::Min));
        }
    }
}