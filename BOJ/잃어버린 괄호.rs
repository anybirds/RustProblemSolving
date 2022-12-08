// BOJ 1541 잃어버린 괄호

use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut l = 0;
    let mut minus = false;
    let mut ans = 0;
    for (i, c) in line.char_indices() {
        if c.is_ascii_digit() {
            continue;
        }

        let mut n = 0;
        if i > 0 {
            n = line[l..i].parse().unwrap();
            l = i + 1;
        }
        
        if minus {
            ans -= n;
        } else {
            ans += n;
        }

        match c {
            '-' => {
                minus = true;
            }
            '+' => (),
            _ => break
        }
    }
    print!("{}", ans);
}