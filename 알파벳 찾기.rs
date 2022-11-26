// BOJ 10809 알파벳 찾기

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line.");

    let line = line.trim();
    let count = line.chars().count();

    let mut ans = [-1; 'z' as usize - 'a' as usize];
    for (i, alphabet) in line.chars().rev().enumerate() {
        ans[alphabet as usize - 'a' as usize] = (count - i - 1) as i32;
    }
    for i in ans {
        print!("{} ", i);
    }
}
