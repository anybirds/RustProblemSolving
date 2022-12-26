// BOJ 1918 후위 표기식

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut stk = Vec::new();
    let mut ans = String::new();
    for char in line.trim().chars() {
        match char {
            '(' => {
                stk.push(char);
            }
            ')' => {
                loop {
                    assert!(!stk.is_empty());
                    match stk.pop().unwrap() {
                        '(' => break,
                        item => ans.push(item)
                    }
                }
            }
            '+' | '-' => {
                loop {
                    if let Some(&item) = stk.last() {
                        match item {
                            '+' | '-' | '*' | '/' => {
                                stk.pop().unwrap(); 
                                ans.push(item);
                            }
                            _ => break
                        }
                    } else {
                        break;
                    }
                }
                stk.push(char);
            }
            '*' | '/' => {
                if let Some(&item) = stk.last() {
                    match item {
                        '*' | '/' => { 
                            stk.pop().unwrap(); 
                            ans.push(item);
                        }
                        _ => ()
                    }
                }
                stk.push(char);
            }
            _ => {
                ans.push(char);
            }
        }
    }
    loop {
        if let Some(item) = stk.pop() {
            ans.push(item);
        } else {
            break;
        }
    }
    print!("{}", ans);
}