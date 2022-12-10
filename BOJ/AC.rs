// BOJ 5430 AC

use std::io;
use std::str::FromStr;
use std::collections::VecDeque;

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
    line.trim().clone().to_string()
}

fn read_arr() -> VecDeque<i32> {
    let line = read_line();
    line.split(&['[', ',', ']'])
        .filter(|s| !s.is_empty())
        .map(|x| -> i32 { match x.parse() { Ok(t) => t, Err(_) => panic!("Failed to parse.") } })
        .collect()
}

fn ac(p: String, mut x: VecDeque<i32>) -> Result<String, ()> {
    let mut rev = false;
    let mut err = false;
    for c in p.chars() {
        match c {
            'R' => {
                rev = !rev;
            }
            'D' => {
                let r;
                if rev {
                    r = x.pop_back();
                } else {
                    r = x.pop_front();
                }
                
                match r {
                    None => {
                        err = true;
                        break;
                    }
                    _ => ()
                }
            }
            _ => panic!("Invalid operation.")
        }
    }
    if err {
        return Err(());
    } else {
        let mut s = String::from("[");
        if rev {
            s.push_str(x.into_iter().rev().map(|x| -> String { x.to_string() }).collect::<Vec<String>>().join(",").as_str());
        } else {
            s.push_str(x.into_iter().map(|x| -> String { x.to_string() }).collect::<Vec<String>>().join(",").as_str());
        }
        
        s.push(']');
        return Ok(s);
    }
}

fn main() {
    let t = read_one();
    for _ in 0..t {
        let p = read_line();
        let n: usize = read_one();
        let x = read_arr();
        
        assert_eq!(n, x.len());
        
        match ac(p, x) {
            Ok(s) => println!("{}", s),
            Err(_) => println!("error")
        }
    }
}